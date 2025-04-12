use std::{future::Future, pin::Pin};

use futures::Stream;

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

    fn fetch_live(&self) -> Pin<Box<dyn Stream<Item = Result<Kline>> + Send>> {
        Box::pin(super::stream::Stream::new())
    }
}
