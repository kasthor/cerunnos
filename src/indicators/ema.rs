use std::f64;

use crate::data_structures::history::History;

use super::Indicator;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct EMAParams {
    pub period: usize,
}

impl EMAParams {
    pub fn name(&self) -> String {
        format!("ema_{}", self.period)
    }
}

pub struct EMA {
    pub params: EMAParams,
}

impl EMA {
    pub fn new(params: EMAParams) -> Self {
        EMA { params }
    }
}

impl Indicator for EMA {
    fn name(&self) -> String {
        self.params.name()
    }
    fn calculate(&self, history: &History) -> Vec<f64> {
        let multiplier = 2.0 / (self.params.period as f64 + 1.0);
        let mut ema_values = Vec::new();
        let mut ema_prev: Option<f64> = None;

        for kline in history.last(self.params.period).iter() {
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

    use crate::{
        data_structures::{history::History, kline::helpers::generate_klines_with_prices},
        indicators::{ema::EMAParams, Indicator},
    };

    use super::EMA;

    #[test]
    fn test_ema_calculation_with_exact_dataset() {
        let prices = vec![100.0, 105.0, 110.0];
        let history = History::with_klines(generate_klines_with_prices(&prices));

        let ema = EMA::new(EMAParams { period: 3 });
        let result = ema.calculate(&history);
        let expected = vec![100.0, 102.5, 106.25];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_calculation_with_fewer_items() {
        let prices = vec![100.0, 105.0];
        let history = History::with_klines(generate_klines_with_prices(&prices));

        let ema = EMA::new(EMAParams { period: 3 });
        let result = ema.calculate(&history);
        let expected = vec![100.0, 102.5];

        assert_eq!(result, expected);
    }
}
