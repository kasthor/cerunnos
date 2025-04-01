use std::f64;

use crate::data_structures::history::History;

use super::Indicator;

pub struct EMA {
    pub period: usize,
    pub name: String,
}

impl EMA {
    pub fn new(name: String, period: usize) -> Self {
        EMA { name, period }
    }
}

impl Indicator for EMA {
    fn name(&self) -> &str {
        &self.name
    }
    fn calculate(&self, history: &History) -> Vec<f64> {
        let multiplier = 2.0 / (self.period as f64 + 1.0);
        let mut ema_values = Vec::new();
        let mut ema_prev: Option<f64> = None;

        for kline in history.last(self.period).iter() {
            let close = kline.close;

            ema_prev = Some(match ema_prev {
                None => close,
                Some(ema_prev) => ((close - ema_prev) * multiplier) + ema_prev,
            });

            ema_values.push(ema_prev.unwrap());
        }

        ema_values
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, Utc};

    use crate::{
        data_structures::{history::History, kline::Kline},
        indicators::Indicator,
    };

    use super::EMA;

    #[test]
    fn test_ema_calculation_with_exact_dataset() {
        let mut history = History::new();
        let prices = vec![100.0, 105.0, 110.0];

        for (i, &close) in prices.iter().enumerate() {
            history.insert(Kline {
                time: (Utc::now() - TimeDelta::try_minutes((prices.len() - i) as i64).unwrap()),
                close,
                ..Default::default()
            })
        }

        let ema = EMA::new("ema".to_string(), 3);
        let result = ema.calculate(&history);
        let expected = vec![100.0, 102.5, 106.25];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_calculation_with_fewer_items() {
        let mut history = History::new();
        let prices = vec![100.0, 105.0];

        for (i, &close) in prices.iter().enumerate() {
            history.insert(Kline {
                time: (Utc::now() - TimeDelta::try_minutes((prices.len() - i) as i64).unwrap()),
                close,
                ..Default::default()
            })
        }

        let ema = EMA::new("ema".to_string(), 3);
        let result = ema.calculate(&history);
        let expected = vec![100.0, 102.5];

        assert_eq!(result, expected);
    }
}
