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
        let mut binance = Binance::new("BTCUSDT".to_string(), "1m".to_string()).await;
        let mut history = History::new();
        let ema = EMA::new("ema".to_string(), 20);
        history.add_calculator(Box::new(ema));

        match binance.fetch_historical_klines().await {
            Ok(klines) => {
                println!("Loaded {} historical klines", klines.len());
                for kline in klines {
                    history.insert(kline);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        while let Some(event) = binance.next().await {
            match event {
                Ok(kline) => {
                    history.insert(Kline::from(kline));
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }
}
