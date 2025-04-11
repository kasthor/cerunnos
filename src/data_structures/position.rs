use chrono::{DateTime, Utc};

use super::signal::Signal;

pub struct Position {
    pub entry_time: DateTime<Utc>,
    pub entry_price: f64,
    pub amount: f64,
    pub symbol: String,
}

impl Position {
    pub fn from_signal(signal: &Signal, amount: f64) -> Self {
        Self {
            entry_time: signal.time,
            entry_price: signal.price,
            amount,
            symbol: signal.symbol.clone(),
        }
    }
}
