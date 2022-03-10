use crate::{rest_client::Client, models::{Channels, OpenContracts}};
use anyhow::{bail, Result};
use log::{debug, error};

pub struct FuturesRESTClient {
    client: Client,
}

impl FuturesRESTClient {
    pub fn new(hostname: &str) -> FuturesRESTClient {
        FuturesRESTClient {
            client: Client::new(hostname.to_string()),
        }
    }

    pub async fn get_public_channels(&self) -> Result<Channels> {
        let result = self.client.post("/bullet-public").await;

        match result {
            Ok(channels) => Ok(channels),
            Err(e) => bail!(format!("Error retrieving channels: {:?}", e)),
        }
    }
}