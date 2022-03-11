use log::{info};
use log4rs;

mod rest_client;
mod futures_rest_client;
mod futures_websocket;
mod models;
mod websocket;

#[tokio::main]
async fn main() {
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");
    info!("We now have nice logging!");

    let client = futures_rest_client::FuturesRESTClient::new("https://api-futures.kucoin.com/api/v1");
    let result = client.get_public_channels().await.unwrap();

    info!("KuCoin bullet-public result: {:?}", result);
}
