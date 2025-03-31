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
    use chrono::{DateTime, TimeDelta, Utc};

    pub(crate) fn generate_klines(start_time: DateTime<Utc>, count: usize) -> Vec<Kline> {
        let prices: Vec<f64> = (0..count).map(|_| 100.0).collect();
        generate_klines_with_interval(start_time, &prices, 1)
    }

    pub(crate) fn generate_klines_with_prices(prices: &[f64]) -> Vec<Kline> {
        let start_time = Utc::now() - TimeDelta::try_minutes((prices.len()) as i64).unwrap();
        generate_klines_with_interval(start_time, prices, 1000)
    }

    pub(crate) fn generate_klines_with_interval(
        start_time: DateTime<Utc>,
        prices: &[f64],
        interval_secs: i64,
    ) -> Vec<Kline> {
        (prices)
            .iter()
            .enumerate()
            .map(|(i, price)| {
                let time = start_time + chrono::Duration::seconds(i as i64 * interval_secs);
                Kline {
                    time,
                    close: *price,
                    ..Default::default()
                }
            })
            .collect()
    }
}
