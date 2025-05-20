use rand::Rng;

pub struct EMACrossoverStrategyParams {
    period: usize,
}

const EMA_PERIOD_MIN: usize = 5;
const EMA_PERIOD_MAX: usize = 50;

impl EMACrossoverStrategyParams {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            period: rng.gen_range(EMA_PERIOD_MIN..=EMA_PERIOD_MAX),
        }
    }
}
