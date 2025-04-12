use std::{error::Error, future::Future, pin::Pin};

use futures::Stream;

use crate::data_structures::kline::Kline;
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub trait Source {
    fn name(&self) -> &str;
    fn symbol(&self) -> &str;
    fn timeframe(&self) -> &str;

    fn fetch_history(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Kline>>> + Send + '_>>;
    fn fetch_live(&self) -> Pin<Box<dyn Stream<Item = Result<Kline>> + Send>>;
}
