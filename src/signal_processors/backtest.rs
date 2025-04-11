use crate::engines::trading::Trading;

use super::SignalProcessor;

pub struct Backtest {
    core: Trading,
}

impl Backtest {
    pub fn new(initial_balance: f64, risk_per_trade: f64, fees: f64) -> Self {
        Self {
            core: Trading::new(initial_balance, risk_per_trade, fees),
        }
    }
}

impl SignalProcessor for Backtest {
    fn process_signal(&mut self, signal: &crate::data_structures::signal::Signal) {
        self.core.process_signal(signal)
    }
}
