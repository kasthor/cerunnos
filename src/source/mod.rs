use std::{error::Error, future::Future, pin::Pin};

use futures::Stream;

use crate::data_structures::kline::Kline;

pub trait Source: Stream<Item = Result<Kline, Box<dyn Error + Send + Sync>>> + Unpin {
    fn name(&self) -> &str;
    fn symbol(&self) -> &str;
    fn timeframe(&self) -> &str;
    fn fetch_history(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Kline>, Box<dyn Error + Send + Sync>>> + Send + '_>>;
}
