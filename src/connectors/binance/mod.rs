mod history;
mod kline;
mod message;
mod source;
pub mod stream;


use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub struct Binance {
    name: String,
    symbol: String,
    interval: String,
}

impl Binance {
    pub async fn new(symbol: String, interval: String) -> Self {
        let name = "binance".to_string();

        Binance { name, symbol, interval }
    }
}
