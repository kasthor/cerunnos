use futures_util::stream::Stream;

use super::{message::KlineEvent, Binance};
use std::{pin::Pin, task::Poll};

impl Stream for Binance {
    type Item = Result<KlineEvent, Box<dyn std::error::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(item)) => {
                let item = match item {
                    Ok(event) => Ok(event),
                    Err(e) => Err(e as Box<dyn std::error::Error>),
                };

                Poll::Ready(Some(item))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
