use futures_websocket::{FuturesWebSockets, SubscribeSymbols};
use log::info;
use log4rs;

mod futures_rest_client;
mod futures_websocket;
mod models;
mod rest_client;
mod websocket;

#[tokio::main]
async fn main() {
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");
    info!("We now have nice logging!");

    let mut kucoin_futures = FuturesWebSockets::new(
        vec!["/contractMarket/tickerV2:".to_string()],
        SubscribeSymbols::All,
    );
    kucoin_futures.run().await.unwrap();
}
