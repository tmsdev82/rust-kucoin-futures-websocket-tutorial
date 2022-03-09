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

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send().await?;

        self.handler(response).await
    }

    fn build_headers(&self) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("crypto-connector"));

        Ok(custom_headers)
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, endpoint);

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers()?)
            .send()
            .await?;

        self.handler(response).await
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
