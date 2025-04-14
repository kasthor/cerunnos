pub mod ema;
pub mod rsi;

use crate::data_structures::history::History;

pub trait Indicator {
    fn name(&self) -> &str;
    fn calculate(&self, history: &History) -> Vec<f64>;
}
