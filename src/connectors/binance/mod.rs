mod history;
mod kline;
mod message;
pub mod stream;

use tokio::{sync::mpsc, task};

use log::{error, trace};
use message::KlineEvent;
use std::{error::Error, time::Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use futures_util::{SinkExt, StreamExt};

const RECONNECT_WAIT: u64 = 5;
const PING_INTERVAL_IN_SECONDS: u64 = 60 * 10;

#[derive(Debug)]
pub struct Binance {
    receiver: mpsc::Receiver<Result<KlineEvent, Box<dyn Error + Send>>>,
    symbol: String,
    interval: String,
}

impl Binance {
    pub async fn new(symbol: String, interval: String) -> Self {
        let url = Url::parse("wss://fstream.binance.com/ws/btcusdt@kline_1m").unwrap();
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            Self::manage_connection(url, tx).await;
        });

        Binance { receiver: rx, symbol, interval}
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
    async fn handle_text_message(text: String, tx: &mpsc::Sender<Result<KlineEvent, Box<dyn Error + Send>>>) -> bool {
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

    async fn manage_connection(url: Url, tx: mpsc::Sender<Result<KlineEvent, Box<dyn Error + Send>>>) {
        loop {
            trace!("connecting to binance websocket");
            match connect_async(&url).await {
                Ok((mut ws_stream, _)) => {
                    trace!("connected to binance websocket");
                    let (ping_task, mut ping_rx) = Binance::ping_handler().await;

                    loop {
                        tokio::select! {
                            _ = ping_rx.recv() => {
                                if let Err(e) = ws_stream.send(Message::Ping(vec![])).await {
                                    error!("error sending ping: {:?}", e )
                                }
                            }

                            msg = ws_stream.next() => {
                                match msg {
                                    Some(Ok(Message::Text(text))) => {
                                        if !Binance::handle_text_message(text, &tx).await {
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
