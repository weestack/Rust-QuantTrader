use polars::frame::DataFrame;
use crate::data::data::{DataHandler, DataHandlerFetch};

#[allow(dead_code)]
#[derive(Debug, Clone)] // Derive necessary traits
pub struct AssetData {
    pub symbol: String,                  // Ticker symbol of the asset
    pub cash: f64,                       // Available cash allocated to this asset
    pub positions: f64,                  // Number of positions held for this asset
    pub position_value: f64,             // Current value of the positions
    pub total_value: f64,                // Total value of the asset (cash + positions)
    pub history: Vec<f64>,               // History of total values over time
    data: Option<DataFrame>,         // DataFrame holding asset-specific price and signal history
}

impl AssetData {
    pub fn new(symbol: &str, cash: f64, positions: f64, position_value: f64) -> Self {
        AssetData {
            symbol: symbol.to_string(),
            cash,
            positions,
            position_value,
            total_value: cash,
            history: vec!(),
            data: None,
        }
    }

    pub fn load_data(&mut self) {
        /* TODO load by coherence with exchange */
        self.data = Some(DataHandler::load_data("examples/data/btcusd_2-min_data.csv"));
    }

    pub fn get_data(&self) -> &Option<DataFrame> {
        &self.data
    }

    pub fn load_latest_candle() {
        /* TODO implement streaming/yielding? */
    }
}