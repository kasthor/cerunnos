use chrono::{DateTime, Duration, Utc};

use super::{position::Position, signal::Signal};

#[derive(Clone)]
pub struct Trade {
    pub symbol: String,
    pub entry_time: DateTime<Utc>,
    pub exit_time: DateTime<Utc>,
    pub entry_price: f64,
    pub exit_price: f64,
    pub amount: f64,
    pub profit_loss: f64,
    pub profit_loss_percent: f64,
}

impl Trade {
    pub fn from_position_and_sell_signal(position: &Position, signal: &Signal) -> Self {
        let position_value = position.amount * signal.price;
        let entry_value = position.amount * position.entry_price;
        let profit_loss = position_value - entry_value;
        let profit_loss_percent = (profit_loss / entry_value) * 100.0;

        Self {
            entry_time: position.entry_time,
            exit_time: signal.time,
            symbol: position.symbol.clone(),
            entry_price: position.entry_price,
            exit_price: signal.price,
            amount: position.amount,
            profit_loss,
            profit_loss_percent,
        }
    }

    pub fn position_value(&self) -> f64 {
        self.exit_price * self.amount
    }

    pub fn entry_value(&self) -> f64 {
        self.entry_price * self.amount
    }

    pub fn duration(&self) -> Duration {
        self.exit_time - self.entry_time
    }
}
