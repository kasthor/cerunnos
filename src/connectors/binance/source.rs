use std::{error::Error, future::Future, pin::Pin};

use tokio::sync::mpsc;
use url::Url;

use crate::{
    data_structures::kline::Kline,
    source::{Result, Source},
};

use super::Binance;

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

    fn is_connected(&self) -> bool {
        self.receiver.is_some()
    }

    fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            if self.is_connected() {
                return Err("already connected".into());
            }

            let url = Url::parse("wss://fstream.binance.com/ws/btcusdt@kline_1m")?;

            let (tx, rx) = mpsc::channel(100);

            self.receiver = Some(rx);

            tokio::spawn(async move {
                Self::manage_connection(url, tx).await;
            });

            Ok(())
        })
    }

    fn disconnect(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            if !self.is_connected() {
                return Err("not connected".into());
            }

            self.receiver = None;

            Ok(())
        })
    }
}
