use chrono::{DateTime, Utc};

use crate::data_structures::kline::Kline;

use super::message::KlineEvent;

impl From<KlineEvent> for Kline {
    fn from(event: KlineEvent) -> Self {
        Self {
            time: DateTime::<Utc>::from_timestamp_millis(event.kline.start_time).expect("failed to parse start time"),
            symbol: event.kline.symbol,
            open: event.kline.open.parse().expect("failed to parse open price"),
            close: event.kline.close.parse().expect("failed to parse close price"),
            low: event.kline.low.parse().expect("failed to parse low price"),
            high: event.kline.high.parse().expect("failed to parse high price"),
            volume: event.kline.volume.parse().expect("failed to parse volume"),
        }
    }
}
