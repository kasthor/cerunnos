use crate::data_structures::signal::{Signal, SignalType};

use super::Strategy;

const LOOK_BACK: usize = 2;

pub struct PriceCrossOverStrategy {
    name: String,
    indicator: String,
}

impl PriceCrossOverStrategy {
    pub fn new(name: String, indicator: String) -> Self {
        Self { name, indicator }
    }

    fn detect_crossover(&self, price_series: &[f64], indicator_series: &[Vec<f64>]) -> Option<SignalType> {
        if price_series.len() < 2 || indicator_series.len() < 2 {
            return None;
        }

        println!("{:?}", price_series);
        let current_price = price_series[price_series.len() - 1];
        let previous_price = price_series[price_series.len() - 2];

        let current_indicator = indicator_series[indicator_series.len() - 1].last()?;
        let previous_indicator = indicator_series[indicator_series.len() - 2].last()?;

        println!(
            "{}, {}, {} ,{}",
            previous_price, *previous_indicator, current_price, *current_indicator
        );

        if previous_price <= *previous_indicator && current_price > *current_indicator {
            Some(SignalType::Buy)
        } else if previous_price >= *previous_indicator && current_price < *current_indicator {
            Some(SignalType::Sell)
        } else {
            Some(SignalType::Hold)
        }
    }
}

impl Strategy for PriceCrossOverStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn generate_signals(
        &self,
        history: &crate::data_structures::history::History,
    ) -> Vec<crate::data_structures::signal::Signal> {
        let mut signals = Vec::new();

        let klines = history.last(LOOK_BACK);

        if klines.len() < 2 {
            return signals;
        }

        let prices: Vec<f64> = klines.iter().map(|k| k.close).collect();

        let indicator_values = history.get_indicator_values(&self.indicator, LOOK_BACK);

        if let Some(signal_type) = self.detect_crossover(&prices, &indicator_values) {
            let latest_kline = &klines[klines.len() - 1];

            signals.push(Signal::new(
                latest_kline.time,
                latest_kline.symbol.clone(),
                signal_type,
                latest_kline.close,
                self.name.clone(),
            ))
        }

        signals
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_structures::{history::History, kline::helpers::generate_klines_with_prices},
        indicators::ema::EMA,
    };

    use super::*;

    #[test]
    fn test_crossover_buy_signal() {
        let mut history = History::new();

        let ema = EMA::new("ema_3".to_string(), 3);
        history.add_calculator(Box::new(ema));

        let prices = vec![100.0, 100.0, 100.0, 101.0];
        let klines = generate_klines_with_prices(&prices);

        for kline in klines {
            history.insert(kline);
        }

        let strategy = PriceCrossOverStrategy::new("PriceCrossEMA".to_string(), "ema_3".to_string());

        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Buy);
    }
    #[test]
    fn test_crossover_sell_signal() {
        let mut history = History::new();

        let ema = EMA::new("ema_3".to_string(), 3);
        history.add_calculator(Box::new(ema));

        let prices = vec![100.0, 100.0, 100.0, 99.0];
        let klines = generate_klines_with_prices(&prices);

        for kline in klines {
            history.insert(kline);
        }

        let strategy = PriceCrossOverStrategy::new("PriceCrossEMA".to_string(), "ema_3".to_string());

        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Sell);
    }
    #[test]
    fn test_crossover_trend_up_hold_signal() {
        let mut history = History::new();

        let ema = EMA::new("ema_3".to_string(), 3);
        history.add_calculator(Box::new(ema));

        let prices = vec![101.0, 102.0, 103.0, 104.0];
        let klines = generate_klines_with_prices(&prices);

        for kline in klines {
            history.insert(kline);
        }

        let strategy = PriceCrossOverStrategy::new("PriceCrossEMA".to_string(), "ema_3".to_string());

        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Hold);
    }
    #[test]
    fn test_crossover_trend_down_hold_signal() {
        let mut history = History::new();

        let ema = EMA::new("ema_3".to_string(), 3);
        history.add_calculator(Box::new(ema));

        let prices = vec![104.0, 103.0, 102.0, 101.0];
        let klines = generate_klines_with_prices(&prices);

        for kline in klines {
            history.insert(kline);
        }

        let strategy = PriceCrossOverStrategy::new("PriceCrossEMA".to_string(), "ema_3".to_string());

        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Hold);
    }
}
