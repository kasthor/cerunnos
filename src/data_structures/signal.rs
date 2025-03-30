use chrono::{DateTime, Utc};

pub enum SignalType {
    Hold,
    Sell,
    Buy,
}

pub struct Signal {
    pub time: DateTime<Utc>,
    pub symbol: String,
    pub signal_type: SignalType,
    pub price: f64,
    pub source: String,
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
