/*
curl --request GET \
     --url 'https://api.coingecko.com/api/v3/coins/orca/ohlc?vs_currency=usd&days=1' \
     --header 'accept: application/json' \
     --header 'x-cg-demo-api-key: <api_key>'

*/

#[allow(dead_code)]
const GECKO_ENDPOINT: &str = "https://api.coingecko.com/api/v3";

#[allow(dead_code)]
/* TODO IMPLEMENT FOR more than Daily usage! */
const OHLCV_ENDPOINT: &str = "coins/orca/ohlc?vs_currency=usd&days=1";


