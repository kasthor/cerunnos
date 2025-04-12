use crate::data_structures::kline::Kline;

use super::message::KlineEvent;
use super::Result;
use std::{pin::Pin, task::Poll};

use log::error;
use log::trace;
use tokio::{sync::mpsc, task};

use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message, Bytes},
};
use url::Url;

const RECONNECT_WAIT: u64 = 5;
const PING_INTERVAL_IN_SECONDS: u64 = 60 * 10;

pub struct Stream {
    receiver: mpsc::Receiver<Result<KlineEvent>>,
}

impl futures_util::stream::Stream for Stream {
    type Item = Result<Kline>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(item)) => {
                let item = match item {
                    Ok(event) => Ok(Kline::from(event)),
                    Err(e) => Err(e as Box<dyn std::error::Error + Send + Sync>),
                };

                Poll::Ready(Some(item))
            }
            Poll::Ready(None) => Poll::Ready(None),

            Poll::Pending => Poll::Pending,
        }
    }
}

impl Stream {
    pub fn new() -> Self {
        let url = Url::parse("wss://fstream.binance.com/ws/btcusdt@kline_1m").unwrap();

        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            Self::manage_connection(url, tx).await;
        });

        Self { receiver: rx }
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
                    let (ping_task, mut ping_rx) = Self::ping_handler().await;

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
                                        if !Self::handle_text_message(text.to_string(), &tx).await {
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
