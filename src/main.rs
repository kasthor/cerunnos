mod connectors;
mod data_structures;
mod indicators;
mod processor;
mod source;
mod strategies;

use connectors::binance::Binance;
use processor::Processor;

#[tokio::main]
async fn main() {
    env_logger::init();

    let source = Box::new(Binance::new("BTCUSDT".to_string(), "1m".to_string()).await);
    let mut processor = Processor::new(source);

    processor.start().await
}
