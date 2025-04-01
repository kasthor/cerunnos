use chrono::{DateTime, Utc};
use log::trace;
use reqwest::Client;
use serde::{de::IgnoredAny, Deserialize};

use crate::data_structures::kline::Kline;

use super::Binance;
use super::Result;

#[derive(Deserialize, Debug)]
struct BinanceHistoryKLine(
    i64,        // Open time
    String,     // Open price
    String,     // High price
    String,     // Low price
    String,     // Close price
    String,     // Volume
    IgnoredAny, // i64,    // Close time
    IgnoredAny, // String, // Quote asset volume
    IgnoredAny, // i64,    // Number of trades
    IgnoredAny, // String, // Taker buy base asset volume
    IgnoredAny, // String, // Taker buy quote asset volume
    IgnoredAny, // String, // Ignore
);

impl From<BinanceHistoryKLine> for Kline {
    fn from(kline: BinanceHistoryKLine) -> Self {
        Self {
            time: DateTime::<Utc>::from_timestamp_millis(kline.0).expect("failed to parse start time"),
            symbol: "".to_string(),
            open: kline.1.parse().expect("failed to parce open price"),
            high: kline.2.parse().expect("failed to parce high price"),
            low: kline.3.parse().expect("failed to parce low price"),
            close: kline.4.parse().expect("failed to parce close price"),
            volume: kline.5.parse().expect("failed to parce volume"),
        }
    }
}

impl Binance {
    pub async fn fetch_historical_klines(&self) -> Result<Vec<Kline>> {
        let client = Client::new();
        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}&interval={}",
            self.symbol, self.interval
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            let error = format!("API error: status: {}, message: {}", status, error_text);
            return Err(error.into());
        }

        let raw_response = response.text().await?;
        let binance_klines: Vec<BinanceHistoryKLine> = match serde_json::from_str(&raw_response) {
            Ok(klines) => klines,
            Err(e) => return Err(Box::new(e)),
        };

        trace!("Loaded {} historical klines", binance_klines.len());

        let klines = binance_klines
            .into_iter()
            .map(|k| {
                let mut kline = Kline::from(k);
                kline.symbol = self.symbol.clone();
                kline
            })
            .collect();

        Ok(klines)
    }
}
