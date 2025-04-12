mod history;
mod kline;
mod message;
mod source;
pub mod stream;

use tokio::{sync::mpsc, task};

use futures_util::{SinkExt, StreamExt};
use message::KlineEvent;
use std::{error::Error, time::Duration};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Bytes},
};
use url::Url;

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
