use futures_util::stream::Stream;

use super::{message::KlineEvent, Binance};
use std::{pin::Pin, task::Poll};
use tokio_tungstenite::tungstenite::protocol::Message;

impl Stream for Binance {
    type Item = Result<KlineEvent, Box<dyn std::error::Error>>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let socket = self.socket.as_mut().unwrap();

        match Pin::new(socket).poll_next(cx) {
            Poll::Ready(Some(Ok(Message::Text(text)))) => match serde_json::from_str::<KlineEvent>(&text) {
                Ok(event) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e.into()))),
            },
            Poll::Ready(Some(Ok(_))) => Poll::Pending,
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
