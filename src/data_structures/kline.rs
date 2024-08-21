use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Kline {
    pub time: DateTime<Utc>,
    pub symbol: String,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub volume: f64,
}

impl Kline {}

impl Default for Kline {
    fn default() -> Self {
        Kline {
            symbol: String::from("UNKNOWN"),
            time: Default::default(),
            open: Default::default(),
            close: Default::default(),
            low: Default::default(),
            high: Default::default(),
            volume: Default::default(),
        }
    }
}

#[cfg(test)]
pub(crate) mod helpers {
    use super::Kline;
    use chrono::{DateTime, Utc};

    pub(crate) fn generate_klines(start_time: DateTime<Utc>, count: usize) -> Vec<Kline> {
        generate_klines_with_interval(start_time, count, 1)
    }

    pub(crate) fn generate_klines_with_interval(
        start_time: DateTime<Utc>,
        count: usize,
        interval_secs: i64,
    ) -> Vec<Kline> {
        (0..count)
            .map(|i| {
                let time = start_time + chrono::Duration::seconds(i as i64 * interval_secs);
                Kline {
                    time,
                    ..Default::default()
                }
            })
            .collect()
    }
}
