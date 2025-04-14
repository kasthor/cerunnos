use crate::data_structures::history::History;

use super::Indicator;

pub struct RSI {
    pub name: String,
    pub period: usize,
}

impl RSI {
    pub fn new(name: String, period: usize) -> Self {
        Self { name, period }
    }
}

impl Indicator for RSI {
    fn name(&self) -> &str {
        &self.name
    }

    fn calculate(&self, history: &History) -> Vec<f64> {
        let klines = history.last(self.period + 1);

        let prices: Vec<f64> = klines.iter().map(|k| k.close).collect();

        self.calculate_rsi_values(&prices)
    }
}

impl RSI {
    fn calculate_rsi_values(&self, prices: &[f64]) -> Vec<f64> {
        if prices.len() <= 1 {
            return Vec::new();
        }

        let changes: Vec<f64> = prices.windows(2).map(|window| window[1] - window[0]).collect();

        let (gains, losses) = changes.iter().fold(
            (Vec::with_capacity(changes.len()), Vec::with_capacity(changes.len())),
            |(mut gains, mut losses), &change| {
                if change >= 0.0 {
                    gains.push(change);
                    losses.push(0.0);
                } else {
                    gains.push(0.0);
                    losses.push(change.abs());
                }

                (gains, losses)
            },
        );

        let mut avg_gain = gains[0..self.period].iter().sum::<f64>() / self.period as f64;
        let mut avg_loss = losses[0..self.period].iter().sum::<f64>() / self.period as f64;

        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        let mut rsi_values = Vec::new();

        rsi_values.push(rsi);

        for i in self.period..gains.len() {
            avg_gain = ((avg_gain * (self.period as f64 - 1.0)) + gains[i]) / self.period as f64;
            avg_loss = ((avg_loss * (self.period as f64 - 1.0)) + losses[i]) / self.period as f64;

            let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }

        rsi_values
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_structures::{history::History, kline::helpers::generate_klines_with_prices},
        indicators::Indicator,
    };

    use super::RSI;

    #[test]
    fn test_rsi_calculation() {
        let prices = vec![
            10.0, 11.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
        ];
        let history = History::with_klines(generate_klines_with_prices(&prices));

        let rsi = RSI::new("rsi_14".to_string(), 14);
        let result = rsi.calculate(&history);

        assert_eq!(result.len(), 1);
        assert!(result[0] >= 0.0 && result[0] < 100.0);
    }

    #[test]
    fn test_rising_rsi() {
        let prices = vec![
            10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0,
        ];
        let history = History::with_klines(generate_klines_with_prices(&prices));

        let rsi = RSI::new("rsi_14".to_string(), 14);
        let result = rsi.calculate(&history);

        assert_eq!(result.len(), 1);
        assert!(result[0] > 70.0);
    }
    #[test]
    fn test_dropping_rsi() {
        let prices = vec![
            24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0,
        ];
        let history = History::with_klines(generate_klines_with_prices(&prices));

        let rsi = RSI::new("rsi_14".to_string(), 14);
        let result = rsi.calculate(&history);

        assert_eq!(result.len(), 1);
        assert!(result[0] < 30.0);
    }
}
