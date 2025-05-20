use rand::Rng;

use crate::{
    engines::generative::mutators::{mutate_f64, mutate_usize},
    strategies::rsi_strategy::RSIStrategyParams,
};

const RSI_PERIOD_MIN: usize = 7;
const RSI_PERIOD_MAX: usize = 28;
const RSI_LEVEL_MIN: f64 = 10.0;
const RSI_LEVEL_MAX: f64 = 40.0;

impl RSIStrategyParams {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let period = rng.gen_range(RSI_PERIOD_MIN..=RSI_PERIOD_MAX);
        let oversold_level = rng.gen_range(RSI_LEVEL_MIN..=RSI_LEVEL_MAX);
        let overbought_level = 100.0 - oversold_level;

        Self {
            period,
            oversold_level,
            overbought_level,
        }
    }

    pub fn mutate(&mut self, mutation_rate: f64, mutation_strength: f64) {
        let mut rng = rand::rng();
        if rng.gen_bool(mutation_rate) {
            self.period = mutate_usize(self.period, RSI_PERIOD_MIN, RSI_PERIOD_MAX, mutation_strength, rng);
        }
        if rng.gen_bool(mutation_rate) {
            self.oversold_level = mutate_f64(
                self.oversold_level,
                RSI_LEVEL_MIN,
                RSI_LEVEL_MAX,
                mutation_strength,
                rng,
            )
        }
        if rng.gen_bool(mutation_rate) {
            self.overbought_level = mutate_f64(
                self.overbought_level,
                100 - RSI_LEVEL_MAX,
                100 - RSI_LEVEL_MIN,
                mutation_strength,
                rng,
            );
        }
    }
}
