use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channels {
    pub code: String,
    pub data: ChannelsData,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsData {
    pub instance_servers: Vec<InstanceServer>,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    pub ping_interval: i64,
    pub endpoint: String,
    pub protocol: String,
    pub encrypt: bool,
    pub ping_timeout: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenContractData {
    pub symbol: String,
    pub root_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub settle_currency: String,
    pub max_order_qty: f64,
    pub max_price: f64,
    pub lot_size: f64,
    pub tick_size: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenContracts {
    pub code: Option<String>,
    pub data: Vec<OpenContractData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenericMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMessage {
    pub id: u64,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub topic: String,
    pub private_channel: bool,
    pub response: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolTicker {
    pub sequence: u64,
    pub symbol: String,
    pub best_bid_price: String,
    pub best_bid_size: u64,
    pub best_ask_price: String,
    pub best_ask_size: u64,
    pub ts: u64,
}
