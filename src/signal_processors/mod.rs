pub mod backtest;
pub mod logging;

use std::any::Any;

use crate::data_structures::signal::Signal;

pub trait SignalProcessor {
    fn process_signal(&mut self, signal: &Signal);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
