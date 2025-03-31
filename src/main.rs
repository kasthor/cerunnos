mod connectors;
mod data_structures;
mod indicators;
mod processor;
mod strategies;

use processor::Processor;

#[tokio::main]
async fn main() {
    env_logger::init();

    let processor = Processor::new();

    processor.start().await
}
