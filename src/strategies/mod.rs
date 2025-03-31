use crate::data_structures::{history::History, signal::Signal};

pub mod crossover;

pub trait Strategy {
    fn name(&self) -> &str;

    fn generate_signals(&self, history: &History) -> Vec<Signal>;
}
