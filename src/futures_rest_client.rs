use crate::rest_client::Client;
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
}