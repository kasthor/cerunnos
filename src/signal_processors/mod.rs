pub mod logging;
pub mod backtest;

use crate::data_structures::signal::Signal;

pub trait SignalProcessor {
    fn process_signal(&mut self, signal: &Signal);
}
