use crate::{
    data_structures::{history::History, signal::Signal},
    indicators::IndicatorIdentifier,
};

pub mod crossover;
pub mod rsi_strategy;

pub trait Strategy {
    fn name(&self) -> &str;
    fn request_indicators(&self) -> Vec<IndicatorIdentifier>;
    fn generate_signals(&self, history: &History) -> Vec<Signal>;
}
