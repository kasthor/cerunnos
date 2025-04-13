pub mod metrics;
use std::{any::Any, collections::HashMap};

use metrics::Metrics;

use log::info;

use crate::{
    data_structures::{
        performance_metrics::PerformanceMetrics,
        position::Position,
        signal::{Signal, SignalType},
        trade::Trade,
    },
    signal_processors::SignalProcessor,
};

pub struct Trading {
    balance: f64,
    risk_per_trade: f64,
    positions: HashMap<String, Position>,
    last_signal: Option<Signal>,
    fees: f64,

    metrics: Metrics,
}

impl Trading {
    pub fn new(balance: f64, risk_per_trade: f64, fees: f64) -> Self {
        Self {
            balance,
            risk_per_trade,
            positions: HashMap::new(),
            last_signal: None,
            fees,

            metrics: Metrics::new(balance),
        }
    }

    pub fn has_position(&self, symbol: &String) -> bool {
        self.positions.contains_key(symbol)
    }

    pub fn execute_buy(&mut self, signal: &Signal) {
        if self.has_position(&signal.symbol) {
            return;
        }

        let position_size = self.balance * self.risk_per_trade;
        let amount = position_size / signal.price;

        let fee_amount = position_size * self.fees;
        self.balance -= position_size - fee_amount;

        self.metrics.update_equity_curve(signal.time, self.balance);

        let position = Position::from_signal(signal, amount);
        self.positions.insert(position.symbol.clone(), position);
    }

    pub fn execute_sell(&mut self, signal: &Signal) {
        if let Some(position) = self.positions.remove(&signal.symbol) {
            let trade = Trade::from_position_and_sell_signal(&position, signal);

            let fee_amount = trade.position_value() * self.fees;
            self.balance += trade.position_value() - fee_amount;

            self.metrics.add_trade(trade.clone());
            self.metrics.update_equity_curve(signal.time, self.balance);

            info!(
                "SELL: {} @ {:.2} | Profit Loss: {:.4} ({:.2}%) | Balance: {:2}",
                trade.symbol, trade.exit_price, trade.profit_loss, trade.profit_loss_percent, self.balance
            )
        }
    }

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.metrics.get_performance_metrics()
    }
}

impl SignalProcessor for Trading {
    fn process_signal(&mut self, signal: &crate::data_structures::signal::Signal) {
        match signal.signal_type {
            SignalType::Buy => self.execute_buy(signal),
            SignalType::Sell => self.execute_sell(signal),
            SignalType::Hold => {}
        }

        self.last_signal = Some(signal.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
