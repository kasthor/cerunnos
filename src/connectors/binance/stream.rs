use futures_util::stream::Stream;

use crate::data_structures::kline::Kline;

use super::Binance;
use super::Result;
use std::{pin::Pin, task::Poll};

impl Stream for Binance {
    type Item = Result<Kline>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(receiver) = &mut self.receiver {
            match receiver.poll_recv(cx) {
                Poll::Ready(Some(item)) => {
                    let item = match item {
                        Ok(event) => Ok(Kline::from(event)),
                        Err(e) => Err(e as Box<dyn std::error::Error + Send + Sync>),
                    };

                    Poll::Ready(Some(item))
                }
                Poll::Ready(None) => {
                    self.receiver = None;
                    Poll::Ready(None)
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        }
    }
}
