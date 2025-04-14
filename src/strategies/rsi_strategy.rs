use crate::data_structures::{
    history::History,
    signal::{Signal, SignalType},
};

use super::Strategy;

const LOOK_BACK: usize = 2;

pub struct RSIStrategy {
    name: String,
    indicator: String,
    overbought_level: f64,
    oversold_level: f64,
}

impl RSIStrategy {
    pub fn new(name: String, indicator: String, overbought_level: f64, oversold_level: f64) -> Self {
        Self {
            name,
            indicator,
            overbought_level,
            oversold_level,
        }
    }

    fn detect_signal(&self, indicator_series: &[Vec<f64>]) -> Option<SignalType> {
        if indicator_series.len() < LOOK_BACK {
            return None;
        }

        let current_rsi = indicator_series[indicator_series.len() - 1].last()?;
        let previous_rsi = indicator_series[indicator_series.len() - 2].last()?;

        match (*previous_rsi, *current_rsi) {
            (prev, curr) if prev <= self.oversold_level && curr > self.oversold_level => Some(SignalType::Buy),
            (prev, curr) if prev >= self.overbought_level && curr < self.overbought_level => Some(SignalType::Sell),
            _ => Some(SignalType::Hold),
        }
    }
}

impl Strategy for RSIStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn generate_signals(&self, history: &History) -> Vec<Signal> {
        let mut signals = Vec::new();

        let klines = history.last(LOOK_BACK);

        if klines.len() < LOOK_BACK {
            return signals;
        }

        let indicator_values = history.get_indicator_values(&self.indicator, LOOK_BACK);

        if let Some(signal_type) = self.detect_signal(&indicator_values) {
            let latest_kline = &klines[klines.len() - 1];

            signals.push(Signal::with_kline(signal_type, self.name.clone(), latest_kline));
        }

        signals
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_structures::{history::History, kline::helpers::generate_klines_with_prices, signal::SignalType},
        indicators::rsi::RSI,
        strategies::Strategy,
    };

    use super::RSIStrategy;

    #[test]
    fn test_rsi_oversold_buy_signal() {
        let prices = vec![
            // falling prices
            100.0, 98.0, 96.0, 94.0, 92.0, 90.0, 88.0, 86.0, 84.0, 82.0, 80.0, 78.0, 76.0, 74.0, 72.0,
            // two consecutive rises intended to trigger a buy signal
            76.0, 80.0, 84.0,
        ];

        let mut history = History::new();
        history.add_calculator(Box::new(RSI::new("rsi_14".to_string(), 14)));

        for kline in generate_klines_with_prices(&prices) {
            history.insert(kline);
        }

        let strategy = RSIStrategy::new("RSIOversold".to_string(), "rsi_14".to_string(), 70.0, 30.0);
        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Buy);
    }

    #[test]
    fn test_rsi_overbought_sell_signal() {
        let prices = vec![
            // rising prices
            100.0, 102.0, 104.0, 106.0, 108.0, 110.0, 112.0, 114.0, 116.0, 118.0, 120.0, 122.0, 124.0, 126.0, 128.0,
            // two consecutive rises intended to trigger a buy signal
            124.0, 120.0, 116.0,
        ];

        let mut history = History::new();
        history.add_calculator(Box::new(RSI::new("rsi_14".to_string(), 14)));

        for kline in generate_klines_with_prices(&prices) {
            history.insert(kline);
        }

        let strategy = RSIStrategy::new("RSIOversold".to_string(), "rsi_14".to_string(), 70.0, 30.0);
        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Sell);
    }

    #[test]
    fn test_rsi_hold_signal() {
        let prices = vec![
            // flat prices
            100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0,
            101.0,
        ];

        let mut history = History::new();
        history.add_calculator(Box::new(RSI::new("rsi_14".to_string(), 14)));

        for kline in generate_klines_with_prices(&prices) {
            history.insert(kline);
        }

        let strategy = RSIStrategy::new("RSIOversold".to_string(), "rsi_14".to_string(), 70.0, 30.0);
        let signals = strategy.generate_signals(&history);

        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Hold);
    }
}
