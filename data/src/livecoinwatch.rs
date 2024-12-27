use std::collections::HashMap;
use std::error::Error;
use reqwest::header;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error as StdError;

/* TODO read from .env */
const LIVE_COIN_WATCH_KEY: &str = "<api_key_here>";
const LIVE_COIN_ENDPOINT: &str = "https://api.livecoinwatch.com";
const HISTORY_ENDPOINT: &str = "coins/single/history";
const COIN_ENDPOINT: &str = "coins/single";

#[allow(dead_code)]
#[derive(Debug)]
pub struct Coin {
    symbol: String,
    currency: String,
    last_worth: f64,
    last_volume: u64,
    last_timestamp: u128,
    history: Vec<CoinSlice>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct CoinSlice {
    date: u64,
    rate: f64,
    volume: u64,
    cap: u64,
    liquidity: u64
}

#[allow(dead_code)]
#[derive(Serialize)]
struct Payload<'a> {
    currency: &'a str,
    code: &'a str,
    meta: bool,
}

#[allow(dead_code)]
impl Coin {

    pub async fn new(symbol: String, currency: String) -> Self {
        let mut coin = Coin {
            symbol,
            currency,
            last_worth: 0f64,
            last_volume: 0u64,
            last_timestamp: 0u128,
            history: Vec::new(),
        };

        let _price = coin.get_latest_price().await;
        println!("price: {:?}", coin);
        coin
    }

    pub async fn update_history(&self) -> Result<(), Box<dyn Error>> {
        let params = [("foo", "bar"), ("baz", "quux")];
        let client = reqwest::Client::new();
        let response = client.post(format!("{LIVE_COIN_ENDPOINT}/{HISTORY_ENDPOINT}"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("x-api-key:", LIVE_COIN_WATCH_KEY)
            .form(&params)
            .send()
            .await?;
        let json = response.json::<HashMap<String, String>>().await?;
        println!("{:?}", json);

        Ok(())
    }

    pub async fn get_latest_price(&mut self) -> Result<f64, Box<dyn StdError>> {
        let payload = Payload {
            currency: "USD",
            code: self.symbol.as_str(),
            meta: false,
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("x-api-key", LIVE_COIN_WATCH_KEY.parse().unwrap());

        let client = reqwest::Client::new();
        let response = client.post(format!("{LIVE_COIN_ENDPOINT}/{COIN_ENDPOINT}"))
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .unwrap();
        // Parse the response JSON
        if response.status() == reqwest::StatusCode::OK {
            let json: HashMap<String, serde_json::Value> = response.json().await?;
            self.last_volume = json.get("volume").unwrap().as_u64().unwrap();
            self.last_worth = json.get("rate").unwrap().as_f64().unwrap();
            self.last_timestamp = now_epoch();
            Ok(())
        } else {
            Err(())
        }.expect("TODO: panic message");

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "An error occurred",
        )))
    }
}

#[allow(dead_code)]
fn now_epoch() -> u128 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}