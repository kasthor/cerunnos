use std::any::Any;

use crate::{data_structures::performance_metrics::PerformanceMetrics, engines::trading::Trading};

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

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.core.get_performance_metrics()
    }
}

impl SignalProcessor for Backtest {
    fn process_signal(&mut self, signal: &crate::data_structures::signal::Signal) {
        self.core.process_signal(signal)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
