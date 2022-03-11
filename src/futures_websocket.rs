use crate::futures_rest_client::FuturesRESTClient;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};
use log::{debug, error, info};
use std::sync::atomic::{AtomicBool, Ordering};

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
        info!("Start event loop...");

        while running.load(Ordering::Relaxed) {
        }
       
        Ok(())
    }

}
