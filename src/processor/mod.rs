use crate::data_structures::history::History;
use crate::indicators::ema::EMA;
use crate::signal_processors::SignalProcessor;
use crate::source::Source;
use crate::strategies::crossover::PriceCrossOverStrategy;
use crate::strategies::Strategy;
use futures_util::StreamExt;
use log::{error, trace};

pub struct Processor {
    history: History,
    strategies: Vec<Box<dyn Strategy>>,
    source: Box<dyn Source>,
    signal_processors: Vec<Box<dyn SignalProcessor>>,
}

impl Processor {
    pub fn new(source: Box<dyn Source>, signal_processors: Vec<Box<dyn SignalProcessor>>) -> Self {
        let mut strategies = Vec::new();
        let mut history = History::new();

        let ema = EMA::new("ema_20".to_string(), 20);
        history.add_calculator(Box::new(ema));

        let price_ema_crossover = PriceCrossOverStrategy::new("EMAPriceCrossOver".to_string(), "ema_20".to_string());

        strategies.push(Box::new(price_ema_crossover) as Box<dyn Strategy>);

        Self {
            source,
            history,
            strategies,
            signal_processors,
        }
    }

    pub async fn start(&mut self) {
        match self.source.fetch_history().await {
            Ok(klines) => {
                for kline in klines {
                    self.history.insert(kline);
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }

        while let Some(event) = self.source.next().await {
            match event {
                Ok(kline) => {
                    self.history.insert(kline);
                    self.apply_strategies().await;
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    pub async fn apply_strategies(&mut self) {
        for strategy in &self.strategies {
            let signals = strategy.generate_signals(&self.history);

            for signal in signals {
                for processor in &mut self.signal_processors {
                    processor.process_signal(&signal);
                }
            }
        }
    }
}
