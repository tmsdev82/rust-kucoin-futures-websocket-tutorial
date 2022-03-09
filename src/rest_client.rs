use anyhow::{bail, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;

pub struct Client {
    host: String,
    inner_client: reqwest::Client,
}

impl Client {
    pub fn new(host: String) -> Self {
        Client {
            host,
            inner_client: reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error: ContentError = response.json().await?;
                bail!(error)
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }

    
}

#[derive(Debug, Deserialize)]
pub struct ContentError {
    pub code: i16,
    pub msg: String,
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {} \nmsg: {}", self.code, self.msg) // user-facing output
    }
}
