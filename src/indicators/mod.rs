pub mod ema;
pub mod factory;
pub mod rsi;

use ema::EMAParams;
use rsi::RSIParams;

use crate::data_structures::history::History;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum IndicatorIdentifier {
    RSI(RSIParams),
    EMA(EMAParams),
}

pub trait Indicator {
    fn name(&self) -> String;
    fn calculate(&self, history: &History) -> Vec<f64>;
}
