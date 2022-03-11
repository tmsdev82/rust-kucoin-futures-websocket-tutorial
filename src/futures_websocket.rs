use crate::{futures_rest_client::FuturesRESTClient, models, websocket};
use anyhow::{bail, Result};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{stream::MaybeTlsStream, Message};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscribeSymbols {
    All,
    Custom(Vec<String>),
}

pub struct FuturesWebSockets {
    exchange: String,
    topics: Vec<String>,                 // Configure the topics to subscribe to.
    subscribe_symbols: SubscribeSymbols, // Configure the symbols to use in topic subscription
    client: FuturesRESTClient,
}

impl FuturesWebSockets {
    pub fn new(topics: Vec<String>, subscribe_symbols: SubscribeSymbols) -> FuturesWebSockets {
        FuturesWebSockets {
            exchange: "kucoin-futures".to_string(),
            topics,
            subscribe_symbols,
            client: FuturesRESTClient::new("https://api-futures.kucoin.com/api/v1"),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let keep_running = AtomicBool::new(true);

        if let Err(e) = self.event_loop(&keep_running).await {
            error!("Error: {}", e);
        }
        info!("[{}] Loop stopped running.", &self.exchange);

        Ok(())
    }

    async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        info!("Establishing connection...");
        let mut socket = match self.connect().await {
            Ok(socket_ok) => socket_ok,
            Err(error) => {
                bail!("error: {}", error)
            }
        };
        info!("Connected!");

        info!("Start event loop...");
        while running.load(Ordering::Relaxed) {
            let message = match socket.0.read_message() {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Error: {}", err);
                    info!("[{}] Reconnecting WebSocket due to error.", &self.exchange);
                    socket = match self.connect().await {
                        Ok(socket) => socket,
                        Err(error) => {
                            bail!("error: {}", error)
                        }
                    };
                    continue;
                }
            };
        }

        Ok(())
    }

    async fn connect(&mut self) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
        let channels: models::Channels = {
            debug!("Retrieving channels information from kucoin");
            let result = self.client.get_public_channels().await;

            match result {
                Ok(channels) => channels,
                Err(e) => bail!("Error occurred: {}", e),
            }
        };

        debug!(
            "Loop through {} server(s)",
            channels.data.instance_servers.len()
        );

        let mut websocket_urls = Vec::new();
        for server in channels.data.instance_servers {
            let connect_id = "_crypto-connector";
            websocket_urls.push(format!(
                "{}?token={}&[connectId={}]",
                &server.endpoint, &channels.data.token, connect_id
            ));
        }

        if let Ok(con) = websocket::connect_wss(&self.exchange, websocket_urls) {
            return Ok(con);
        }

        bail!("Unable to connect.");
    }
}
