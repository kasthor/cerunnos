use super::{ema::EMA, rsi::RSI, Indicator, IndicatorIdentifier};

pub struct Factory {}

impl Factory {
    pub fn create(indicator: &IndicatorIdentifier) -> Box<dyn Indicator> {
        match indicator {
            IndicatorIdentifier::RSI(params) => Box::new(RSI::new(params.clone())),
            IndicatorIdentifier::EMA(params) => Box::new(EMA::new(params.clone())),
        }
    }
}
