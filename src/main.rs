mod connectors;
mod data_structures;
mod indicators;
mod processor;

use processor::Processor;

#[tokio::main]
async fn main() {
    let processor = Processor::new();

    processor.start().await
}
