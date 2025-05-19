use crate::data_structures::history::History;
use crate::data_structures::kline::Kline;
use crate::indicators::ema::EMAParams;
use crate::indicators::IndicatorIdentifier;
use crate::signal_processors::backtest::Backtest;
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

enum StrategyOption {
    Apply,
    Skip,
}

pub enum ProcessorMode {
    Backtest,
    Live,
}

impl Processor {
    pub fn new(source: Box<dyn Source>, signal_processors: Vec<Box<dyn SignalProcessor>>) -> Self {
        let mut strategies = Vec::new();
        let mut history = History::new();
        let ema_20 = IndicatorIdentifier::EMA(EMAParams { period: 20 });

        let price_ema_crossover = PriceCrossOverStrategy::new("EMAPriceCrossOver".to_string(), ema_20.clone());

        strategies.push(Box::new(price_ema_crossover) as Box<dyn Strategy>);

        history.request_calculators(
            strategies
                .iter()
                .flat_map(|strategy| strategy.request_indicators())
                .collect::<Vec<IndicatorIdentifier>>()
                .as_slice(),
        );

        Self {
            source,
            history,
            strategies,
            signal_processors,
        }
    }

    pub async fn start(&mut self, mode: ProcessorMode) -> Result<()> {
        match self.source.fetch_history().await {
            Ok(klines) => {
                let strategy_option = match mode {
                    ProcessorMode::Live => StrategyOption::Skip,
                    ProcessorMode::Backtest => StrategyOption::Apply,
                };
                let historical_data = stream::iter(klines.into_iter().map(Ok));
                self.consume_klines(historical_data, strategy_option).await?;
            }
            Err(e) => error!("{}", e),
        }

        match mode {
            ProcessorMode::Live => {
                let stream = self.source.fetch_live();
                self.consume_klines(stream, StrategyOption::Apply).await
            }
            ProcessorMode::Backtest => {
                if let Some(backtest) = Self::get_signal_processor_by_type::<Backtest>(&mut self.signal_processors) {
                    backtest.get_performance_metrics().print_summary();
                }

                Ok(())
            }
        }
    }

    async fn consume_klines<T>(&mut self, stream: T, strategy: StrategyOption) -> Result<()>
    where
        T: StreamExt<Item = Result<Kline>>,
    {
        tokio::pin!(stream);

        while let Some(event) = stream.next().await {
            match event {
                Ok(kline) => {
                    self.history.insert(kline);
                    if let StrategyOption::Apply = strategy {
                        self.apply_strategies().await
                    }
                }
                Err(e) => error!("while processing kline: {:?}", e),
            }
        }

        Ok(())
    }

    async fn apply_strategies(&mut self) {
        for strategy in &self.strategies {
            let signals = strategy.generate_signals(&self.history);

            for signal in signals {
                for processor in &mut self.signal_processors {
                    processor.process_signal(&signal);
                }
            }
        }
    }

    fn get_signal_processor_by_type<T: 'static>(processors: &mut [Box<dyn SignalProcessor>]) -> Option<&T> {
        for processor in processors {
            if let Some(typed_processor) = processor.as_any().downcast_ref::<T>() {
                return Some(typed_processor);
            }
        }

        None
    }
}
