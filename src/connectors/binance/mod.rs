mod history;
mod kline;
mod message;
pub mod stream;

use tokio::{sync::mpsc, task};

use log::{error, trace};
use message::KlineEvent;
use std::{error::Error, future::Future, pin::Pin, time::Duration};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Bytes},
};
use url::Url;

use futures_util::{SinkExt, StreamExt};

use crate::{data_structures::kline::Kline, source::Source};

const RECONNECT_WAIT: u64 = 5;
const PING_INTERVAL_IN_SECONDS: u64 = 60 * 10;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub struct Binance {
    name: String,
    receiver: mpsc::Receiver<Result<KlineEvent>>,
    symbol: String,
    interval: String,
}

impl Source for Binance {
    fn name(&self) -> &str {
        &self.name
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn timeframe(&self) -> &str {
        &self.interval
    }

    fn fetch_history(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Kline>>> + Send + '_>> {
        Box::pin(async move { self.fetch_historical_klines().await })
    }
}

impl Binance {
    pub async fn new(symbol: String, interval: String) -> Self {
        let name = "binance".to_string();
        let url = Url::parse("wss://fstream.binance.com/ws/btcusdt@kline_1m").unwrap();
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            Self::manage_connection(url, tx).await;
        });

        Binance {
            name,
            receiver: rx,
            symbol,
            interval,
        }
    }
    async fn ping_handler() -> (task::JoinHandle<()>, mpsc::Receiver<()>) {
        let (tx, rx) = mpsc::channel(1);

        (
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(PING_INTERVAL_IN_SECONDS));

                loop {
                    interval.tick().await;

                    if tx.send(()).await.is_err() {
                        break;
                    }
                }
            }),
            rx,
        )
    }
    async fn handle_text_message(text: String, tx: &mpsc::Sender<Result<KlineEvent>>) -> bool {
        match serde_json::from_str::<KlineEvent>(&text) {
            Ok(event) => {
                if tx.send(Ok(event)).await.is_err() {
                    return false;
                }
            }
            Err(e) => {
                if tx.send(Err(Box::new(e))).await.is_err() {
                    return false;
                }
            }
        }
        true
    }

    async fn manage_connection(url: Url, tx: mpsc::Sender<Result<KlineEvent>>) {
        loop {
            trace!("connecting to binance websocket");
            match connect_async(&url.to_string()).await {
                Ok((mut ws_stream, _)) => {
                    trace!("connected to binance websocket");
                    let (ping_task, mut ping_rx) = Binance::ping_handler().await;

                    loop {
                        tokio::select! {
                            _ = ping_rx.recv() => {
                                if let Err(e) = ws_stream.send(Message::Ping(Bytes::new())).await {
                                    error!("error sending ping: {:?}", e )
                                }
                            }

                            msg = ws_stream.next() => {
                                match msg {
                                    Some(Ok(Message::Text(text))) => {
                                        if !Binance::handle_text_message(text.to_string(), &tx).await {
                                            break
                                        }
                                    }
                                    Some(Ok(Message::Ping(data))) => {
                                        if let Err(e) = ws_stream.send(Message::Pong(data)).await {
                                            error!("error sending pong: {:?}", e );
                                            break;
                                        }
                                    }
                                    Some(Ok(Message::Close(_))) => {
                                        error!("received close from server");
                                        break;
                                    }
                                    Some(Err(e)) => {
                                        error!("web socket error: {:?}", e);
                                        let _ = tx.send(Err(Box::new(e))).await.is_err();
                                        break;
                                    },
                                    None => {
                                        error!("websocket stream ended");
                                        break
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }

                    ping_task.abort();
                }
                Err(e) => {
                    error!("connection to binance has failed: {:?}", e);
                }
            }

            tokio::time::sleep(Duration::from_secs(RECONNECT_WAIT)).await
        }
    }
}
