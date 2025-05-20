use crate::indicators::ema::EMAParams;

use super::{crossover::PriceCrossOverStrategy, rsi_strategy::RSIStrategyParams};

struct EMACrossoverStrategyParams {
    period: usize,
}

pub enum StrategyIdentifier {
    EMACrossoverStrategy(EMACrossoverStrategyParams),
    RSIStrategy(RSIStrategyParams),
}

pub struct Factory {}

impl Factory {
    pub fn create(strategy: &StrategyIdentifier) -> Box<dyn Strategy> {
        match strategy {
            StrategyIdentifier::EMACrossoverStrategy(params) => PriceCrossOverStrategy::new(
                "EmaCrossover".to_string(),
                crate::indicators::IndicatorIdentifier::EMA(EMAParams { period: params.period }),
            ),
            StrategyIdentifier::RSIStrategy(params) => RSIStrategyParams::new("RSI".to_string(), params),
        }
    }
}
