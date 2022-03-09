use log::{info};
use log4rs;

mod rest_client;

#[tokio::main]
async fn main() {
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");
    info!("We now have nice logging!");

    let client = rest_client::Client::new("https://api-futures.kucoin.com/api/v1".to_string());
    let result: serde_json::Value = client.post("/bullet-public").await.unwrap();

    info!("KuCoin bullet-public result: {:?}", result);
}
