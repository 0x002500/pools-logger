use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Turbos {
    code: i64,
    message: String,
    data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    list: Vec<List>,
    total: i64,
    page: i64,
    page_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    id: i64,
    coin_a: String,
    coin_b: String,
    deploy_time_ms: String,
    fee: String,
    fee_growth_global_a: String,
    fee_growth_global_b: String,
    fee_protocol: String,
    liquidity: String,
    max_liquidity_per_tick: String,
    protocol_fees_a: String,
    protocol_fees_b: String,
    sqrt_price: String,
    tick_current_index: i64,
    tick_spacing: String,
    unlocked: bool,
    pool_id: String,
    #[serde(rename = "type")]
    list_type: String,
    coin_symbol_a: String,
    coin_symbol_b: String,
    coin_type_a: String,
    coin_type_b: String,
    fee_type: String,
    #[serde(rename = "add_2_percent_depth")]
    add_2__percent_depth: String,
    #[serde(rename = "reduce_2_percent_depth")]
    reduce_2__percent_depth: String,
    reward_last_updated_time_ms: String,
    category: Option<String>,
    apr: f64,
    #[serde(rename = "apr_7d")]
    apr_7_d: f64,
    #[serde(rename = "fee_7d_apr")]
    fee_7_d_apr: f64,
    apr_percent: i64,
    fee_apr: f64,
    reward_apr: f64,
    #[serde(rename = "volume_24h_usd")]
    volume_24_h_usd: f64,
    #[serde(rename = "volume_7d_usd")]
    volume_7_d_usd: f64,
    liquidity_usd: f64,
    created_at: String,
    updated_at: String,
    coin_a_liquidity_usd: f64,
    coin_b_liquidity_usd: f64,
    #[serde(rename = "fee_24h_usd")]
    fee_24_h_usd: f64,
    #[serde(rename = "fee_7d_usd")]
    fee_7_d_usd: f64,
    #[serde(rename = "reward_7d_apr")]
    reward_7_d_apr: f64,
    flag: i64,
    reward_infos: Vec<RewardInfo>,
    is_vault: bool,
    top1_pool: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardInfo {
    #[serde(rename = "type")]
    reward_info_type: String,
    fields: Fields,
}

#[derive(Serialize, Deserialize)]
pub struct Fields {
    id: Id,
    vault: String,
    manager: String,
    growth_global: String,
    vault_coin_type: String,
    emissions_per_second: String,
}

#[derive(Serialize, Deserialize)]
pub struct Id {
    id: String,
}
