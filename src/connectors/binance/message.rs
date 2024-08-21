use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineEvent {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    time: i64,
    #[serde(rename = "s")]
    symbol: String,

    #[serde(rename = "k")]
    pub kline: KlineDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineDetail {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub fist_trade_id: u32,
    #[serde(rename = "L")]
    pub last_trade_id: u32,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub trade_count: u32,
    #[serde(rename = "x")]
    pub closed: bool,
    #[serde(rename = "q")]
    pub quote: String,
    #[serde(rename = "V")]
    pub active_volume: String,
    #[serde(rename = "Q")]
    pub active_quote: String,
}
