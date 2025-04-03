use crate::data_structures::signal::Signal;
pub mod logging;

pub trait SignalProcessor {
    fn process_signal(&mut self, signal: &Signal);
}
