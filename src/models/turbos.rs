use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Turbos {
    pub code: i64,
    pub message: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub list: Vec<List>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    pub id: i64,
    pub coin_a: String,
    pub coin_b: String,
    pub deploy_time_ms: String,
    pub fee: String,
    pub fee_growth_global_a: String,
    pub fee_growth_global_b: String,
    pub fee_protocol: String,
    pub liquidity: String,
    pub max_liquidity_per_tick: String,
    pub protocol_fees_a: String,
    pub protocol_fees_b: String,
    pub sqrt_price: String,
    pub tick_current_index: i64,
    pub tick_spacing: String,
    pub unlocked: bool,
    pub pool_id: String,
    #[serde(rename = "type")]
    pub list_type: String,
    pub coin_symbol_a: String,
    pub coin_symbol_b: String,
    pub coin_type_a: String,
    pub coin_type_b: String,
    pub fee_type: String,
    #[serde(rename = "add_2_percent_depth")]
    pub add_2_percent_depth: String,
    #[serde(rename = "reduce_2_percent_depth")]
    pub reduce_2_percent_depth: String,
    pub reward_last_updated_time_ms: String,
    pub category: Option<String>,
    pub apr: f64,
    #[serde(rename = "apr_7d")]
    pub apr_7_d: f64,
    #[serde(rename = "fee_7d_apr")]
    pub fee_7_d_apr: f64,
    pub apr_percent: i64,
    pub fee_apr: f64,
    pub reward_apr: f64,
    #[serde(rename = "volume_24h_usd")]
    pub volume_24_h_usd: f64,
    #[serde(rename = "volume_7d_usd")]
    pub volume_7_d_usd: f64,
    pub liquidity_usd: f64,
    pub created_at: String,
    pub updated_at: String,
    pub coin_a_liquidity_usd: f64,
    pub coin_b_liquidity_usd: f64,
    #[serde(rename = "fee_24h_usd")]
    pub fee_24_h_usd: f64,
    #[serde(rename = "fee_7d_usd")]
    pub fee_7_d_usd: f64,
    #[serde(rename = "reward_7d_apr")]
    pub reward_7_d_apr: f64,
    pub flag: i64,
    pub reward_infos: Vec<RewardInfo>,
    pub is_vault: bool,
    pub top1_pool: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardInfo {
    #[serde(rename = "type")]
    pub reward_info_type: String,
    pub fields: Fields,
}

#[derive(Serialize, Deserialize)]
pub struct Fields {
    pub id: Id,
    pub vault: String,
    pub manager: String,
    pub growth_global: String,
    pub vault_coin_type: String,
    pub emissions_per_second: String,
}

#[derive(Serialize, Deserialize)]
pub struct Id {
    pub id: String,
}
