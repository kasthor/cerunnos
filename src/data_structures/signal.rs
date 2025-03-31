use core::fmt;
use std::fmt::Debug;

use chrono::{DateTime, Utc};
use env_logger::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum SignalType {
    Hold,
    Sell,
    Buy,
}
impl fmt::Display for SignalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SignalType::Buy => write!(f, "buy"),
            SignalType::Sell => write!(f, "sell"),
            SignalType::Hold => write!(f, "sell"),
        }
    }
}

#[derive(Debug)]
pub struct Signal {
    pub time: DateTime<Utc>,
    pub symbol: String,
    pub signal_type: SignalType,
    pub price: f64,
    pub source: String,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Signal ({}, {} ,{}, Price {:.2})",
            self.time, self.symbol, self.signal_type, self.price
        )
    }
}

impl Signal {
    pub fn new(time: DateTime<Utc>, symbol: String, signal_type: SignalType, price: f64, source: String) -> Self {
        Signal {
            time,
            symbol,
            signal_type,
            price,
            source,
        }
    }

    pub fn buy(time: DateTime<Utc>, symbol: String, price: f64, source: String) -> Self {
        Self::new(time, symbol, SignalType::Buy, price, source)
    }
    pub fn sell(time: DateTime<Utc>, symbol: String, price: f64, source: String) -> Self {
        Self::new(time, symbol, SignalType::Sell, price, source)
    }
    pub fn hold(time: DateTime<Utc>, symbol: String, price: f64, source: String) -> Self {
        Self::new(time, symbol, SignalType::Hold, price, source)
    }
}
