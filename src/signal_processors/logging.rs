use log::trace;

use crate::data_structures::signal::Signal;

use super::SignalProcessor;

pub struct Logging {
    last_signal: Option<Signal>,
}

impl Logging {
    pub fn new() -> Self {
        Self { last_signal: None }
    }
}

impl SignalProcessor for Logging {
    fn process_signal(&mut self, signal: &Signal) {
        if let Some(last_signal) = &self.last_signal {
            if last_signal.signal_type != signal.signal_type {
                trace!("Signal updated: {:?}", signal);
            }
        }
        self.last_signal = Some(signal.clone());
    }
}
