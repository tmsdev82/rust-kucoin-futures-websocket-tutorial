use crate::{
    models::{Channels, OpenContracts},
    rest_client::Client,
};
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

    pub async fn get_open_contracts(&self) -> Result<OpenContracts> {
        let result = self.client.get("/contracts/active", None).await;

        match result {
            Ok(open_contracts) => Ok(open_contracts),
            Err(e) => bail!(format!("Error retrieving contracts: {:?}", e)),
        }
    }

    pub async fn get_available_symbols(&self) -> Result<Vec<String>> {
        let contracts = match self.get_open_contracts().await {
            Ok(contracts) => contracts,
            Err(e) => {
                bail!(e);
            }
        };

        let symbols_list: Vec<String> = contracts.data.iter().map(|f| f.symbol.clone()).collect();
        debug!("Found {} available symbols.", symbols_list.len());

        Ok(symbols_list)
    }
}
