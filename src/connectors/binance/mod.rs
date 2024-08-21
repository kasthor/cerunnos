mod kline;
mod message;
pub mod stream;

use tokio_tungstenite::connect_async;
use url::Url;

#[derive(Debug)]
pub struct Binance {
    socket: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl Binance {
    pub async fn new() -> Self {
        let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@kline_1m").unwrap();
        let (socket, _) = connect_async(url).await.expect("Failed to connect");

        Binance { socket: Some(socket) }
    }
}
