mod connectors;
mod data_structures;
mod indicators;
mod processor;
mod signal_processors;
mod source;
mod strategies;
mod engines;

use connectors::binance::Binance;
use processor::Processor;
use signal_processors::SignalProcessor;

#[tokio::main]
async fn main() {
    env_logger::init();

    let source = Box::new(Binance::new("BTCUSDT".to_string(), "1m".to_string()).await);
    let logging_signal_processor = Box::new(signal_processors::logging::Logging::new());
    let backtest_signal_processor = Box::new(signal_processors::backtest::Backtest::new(1000.0, 0.1, 0.05));
    let mut signal_processors: Vec<Box<dyn SignalProcessor>> = Vec::new();

    signal_processors.push(logging_signal_processor);
    signal_processors.push(backtest_signal_processor);
    let mut processor = Processor::new(source, signal_processors);

    processor.start().await
}
