use rand::Rng;

use crate::strategies::{factory::StrategyIdentifier, rsi_strategy::RSIStrategyParams};

use super::strategies::crossover::EMACrossoverStrategyParams;

impl StrategyIdentifier {
    pub fn random() -> Self {
        let rng = rand::rng();
        match rng.gen_range(0..2) {
            0 => StrategyIdentifier::EMACrossoverStrategy(EMACrossoverStrategyParams::random()),
            1 => StrategyIdentifier::RSIStrategy(RSIStrategyParams::random()),
            _ => unreachable!(),
        }
    }
}
