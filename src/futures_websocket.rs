use crate::{futures_rest_client::FuturesRESTClient, models, websocket};
use anyhow::{bail, Result};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{stream::MaybeTlsStream, Message};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum FuturesEvents {
    BookTickerEvent(models::SymbolTicker),
    GenericMessageEvent(models::GenericMessage), // KuCoin sends generic message of this form
    Unknown,
}

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
            match message {
                Message::Text(msg) => {
                    if let Err(e) = self.handle_msg(&msg, &mut socket.0).await {
                        error!("Error on handling stream message: {}", e);
                        continue;
                    }
                }
                // We can ignore these message because tungstenite takes care of them for us.
                Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => (),
                Message::Close(e) => {
                    error!("Disconnected {:?}", e);
                    continue;
                }
            }
        }
        socket.0.close(None)?;
        Ok(())
    }

    async fn handle_msg(
        &mut self,
        msg: &str,
        socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let mut value: serde_json::Value = serde_json::from_str(msg)?;

        loop {
            value = match value.get("data") {
                Some(data) => serde_json::from_str(&data.to_string())?,
                None => break,
            };
        }

        if let Ok(events) = serde_json::from_value::<FuturesEvents>(value) {
            match events {
                FuturesEvents::BookTickerEvent(v) => {
                    return self.handle_symbol_ticker_event(v).await;
                }
                FuturesEvents::GenericMessageEvent(v) => match v.msg_type.as_str() {
                    "welcome" => {
                        info!("Welcome message received, Subscribing to bookticker...");
                        self.subscribe_to_topics(socket).await;
                    }
                    _ => {
                        info!("Generic message received: {}", &v.msg_type);
                    }
                },
                _ => {
                    info!(
                        "Generic event conversion not yet implemented for: FuturesEvents::{:?}",
                        events
                    );
                    return Ok(());
                }
            };
        } else {
            error!("Unknown message {}", msg);
        }
        Ok(())
    }

    async fn subscribe_to_topics(&self, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
        let symbols: Vec<String> = match &self.subscribe_symbols {
            SubscribeSymbols::All => self.client.get_available_symbols().await.unwrap(),
            SubscribeSymbols::Custom(symbols) => symbols.clone(),
        };

        let mut topics_clone = self.topics.clone();
        let mut current_topic = topics_clone.pop();
        while let Some(topic) = current_topic.clone() {
            for symbol in &symbols {
                if socket.can_write() {
                    let subscribe_topic = format!("{}{}", topic, symbol);
                    info!(
                        "[{}] Subscribing to topic with symbol: {}", &self.exchange,
                        &subscribe_topic
                    );
                    let msg = models::SubscribeMessage {
                        id: 1,
                        msg_type: "subscribe".to_string(),
                        topic: subscribe_topic,
                        private_channel: false,
                        response: true,
                    };
                    let json = serde_json::to_string(&msg).unwrap();
                    let message = Message::Text(json);
                    socket.write_message(message).unwrap();
                } else {
                    error!("Cannot write to socket.");
                }
            }
            current_topic = topics_clone.pop();
        }
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
