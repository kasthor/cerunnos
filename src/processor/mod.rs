use crate::connectors::binance::Binance;
use crate::data_structures::history::History;
use crate::data_structures::kline::Kline;
use crate::indicators::ema::EMA;
use futures_util::StreamExt;

pub struct Processor {}

impl Processor {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn start(self) {
        let mut binance = Binance::new().await;
        let mut history = History::new();
        let ema = EMA::new("ema".to_string(), 5);
        history.add_calculator(Box::new(ema));

        while let Some(event) = binance.next().await {
            match event {
                Ok(kline) => {
                    history.insert(Kline::from(kline));
                    println!("{:?}", history);
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }
}
