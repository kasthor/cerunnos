use crate::data_structures::history::History;
use crate::data_structures::kline::Kline;
use crate::indicators::ema::EMA;
use crate::signal_processors::SignalProcessor;
use crate::source::{Result, Source};
use crate::strategies::crossover::PriceCrossOverStrategy;
use crate::strategies::Strategy;
use futures::stream::{self, StreamExt};
use log::error;

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

    pub async fn start(&mut self) -> Result<()> {
        match self.source.fetch_history().await {
            Ok(klines) => {
                let historical_data = stream::iter(klines.into_iter().map(Ok));
                self.consume_klines(historical_data, false).await?;
            }
            Err(e) => error!("{}", e),
        }

        let stream = self.source.fetch_live();

        self.consume_klines(stream, true).await
    }

    pub async fn consume_klines<T>(&mut self, stream: T, apply_strategies: bool) -> Result<()>
    where
        T: StreamExt<Item = Result<Kline>>,
    {
        tokio::pin!(stream);

        while let Some(event) = stream.next().await {
            match event {
                Ok(kline) => {
                    self.history.insert(kline);
                    if apply_strategies {
                        self.apply_strategies().await
                    }
                }
                Err(e) => error!("while processing kline: {:?}", e),
            }
        }

        Ok(())
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
