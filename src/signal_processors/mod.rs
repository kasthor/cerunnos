pub mod backtest;
pub mod logging;

use crate::data_structures::signal::Signal;

pub trait SignalProcessor {
    fn process_signal(&mut self, signal: &Signal);
}
