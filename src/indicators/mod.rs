pub mod ema;
pub mod rsi;

use ema::EMAParams;
use rsi::RSIParams;

use crate::data_structures::history::History;

enum IndicatorIdentifier {
    RSI(RSIParams),
    EMA(EMAParams),
}

pub trait Indicator {
    fn name(&self) -> &str;
    fn calculate(&self, history: &History) -> Vec<f64>;
}
