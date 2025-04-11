pub mod metrics;
use std::collections::HashMap;

use log::info;

use crate::{
    data_structures::{
        position::Position,
        signal::{Signal, SignalType},
        trade::Trade,
    },
    signal_processors::SignalProcessor,
};

pub struct Trading {
    initial_balance: f64,
    balance: f64,
    risk_per_trade: f64,
    positions: HashMap<String, Position>,
    completed_trades: Vec<Trade>,
    last_signal: Option<Signal>,
    fees: f64,
}

impl Trading {
    pub fn new(initial_balance: f64, risk_per_trade: f64, fees: f64) -> Self {
        Self {
            initial_balance,
            balance: initial_balance,
            risk_per_trade,
            completed_trades: Vec::new(),
            positions: HashMap::new(),
            last_signal: None,
            fees,
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

        let position = Position::from_signal(signal, amount);
        self.positions.insert(position.symbol.clone(), position);
    }

    pub fn execute_sell(&mut self, signal: &Signal) {
        if let Some(position) = self.positions.remove(&signal.symbol) {
            let trade = Trade::from_position_and_sell_signal(&position, signal);

            let fee_amount = trade.position_value() * self.fees;
            self.balance += trade.position_value() - fee_amount;

            self.completed_trades.push(trade.clone());

            info!(
                "SELL: {} @ {:.2} | Profit Loss: {:.4} ({:.2}%) | Balance: {:2}",
                trade.symbol, trade.exit_price, trade.profit_loss, trade.profit_loss_percent, self.balance
            )
        }
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
}
