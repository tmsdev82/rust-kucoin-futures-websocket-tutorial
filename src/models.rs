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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenContractData {
    pub symbol: String,
    pub root_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub first_open_date: u64,
    pub expire_date: Option<u64>,
    pub settle_date: Option<u64>,
    pub base_currency: String,
    pub quote_currency: String,
    pub settle_currency: String,
    pub max_order_qty: f64,
    pub max_price: f64,
    pub lot_size: f64,
    pub tick_size: f64,
    pub index_price_tick_size: f64,
    pub multiplier: f64,
    pub initial_margin: f64,
    pub maintain_margin: f64,
    pub max_risk_limit: f64,
    pub min_risk_limit: f64,
    pub risk_step: f64,
    pub maker_fee_rate: f64,
    pub taker_fee_rate: f64,
    pub taker_fix_fee: f64,
    pub maker_fix_fee: f64,
    pub settlement_fee: Option<f64>,
    pub is_deleverage: bool,
    pub is_quanto: bool,
    pub is_inverse: bool,
    pub mark_method: String,
    pub fair_method: Option<String>,
    pub funding_base_symbol: Option<String>,
    pub funding_quote_symbol: Option<String>,
    pub funding_rate_symbol: Option<String>,
    pub index_symbol: String,
    pub settlement_symbol: Option<String>,
    pub status: String,
    pub funding_fee_rate: Option<f64>,
    pub predicted_funding_fee_rate: Option<f64>,
    pub open_interest: String,
    pub turnover_of24h: f64,
    pub volume_of24h: f64,
    pub mark_price: f64,
    pub index_price: f64,
    pub last_trade_price: f64,
    pub next_funding_rate_time: Option<u64>,
    pub max_leverage: u64,
    pub source_exchanges: Vec<String>,
    pub premiums_symbol1_m: String,
    pub premiums_symbol8_h: String,
    pub funding_base_symbol1_m: Option<String>,
    pub funding_quote_symbol1_m: Option<String>,
    pub low_price: f64,
    pub high_price: f64,
    pub price_chg_pct: f64,
    pub price_chg: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenContracts {
    pub code: Option<String>,
    pub data: Vec<OpenContractData>,
}
