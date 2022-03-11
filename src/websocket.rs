use anyhow::{bail, Result};
use log::{error, info};
use std::net::TcpStream;
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

pub fn connect_wss(
    exchange: &str,
    websocket_urls: Vec<String>,
) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
    let max_retry = 5;
    for i in 0..max_retry {
        for wss in &websocket_urls {
            info!("[{}] connecting to {} (try {})", exchange, wss, i);
            let url = Url::parse(wss)?;
        
            match tungstenite::connect(url) {
                Ok(answer) => {
                    return Ok(answer);
                }
                Err(e) => error!("Error during handshake {}", e),
            }
        }
    }

    bail!(format!("Max connection retry reached"));
}
