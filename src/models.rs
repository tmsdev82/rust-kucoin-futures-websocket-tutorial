use serde::Deserialize;

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
