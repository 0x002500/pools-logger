use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cetus {
    pub code: i64,
    pub msg: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub total: i64,
    pub lp_list: Vec<LpList>,
}

#[derive(Serialize, Deserialize)]
pub struct LpList {
    pub symbol: String,
    pub name: String,
    pub decimals: i64,
    pub fee: String,
    pub tick_spacing: String,
    pub pool_type: String,
    pub address: String,
    pub coin_a_address: String,
    pub coin_b_address: String,
    pub project_url: String,
    pub is_display_rewarder: bool,
    pub is_closed: bool,
    pub rewarder_display1: bool,
    pub rewarder_display2: bool,
    pub rewarder_display3: bool,
    pub rewarder_display4: bool,
    pub rewarder_display5: bool,
    pub labels: Option<serde_json::Value>,
    pub coin_a: Coin,
    pub coin_b: Coin,
    pub price: String,
    pub rewarder_usd: Vec<Option<serde_json::Value>>,
    pub rewarder_apr: Vec<String>,
    pub is_forward: bool,
    pub price_range_config: Option<serde_json::Value>,
    pub object: Object,
    pub category: String,
    pub is_vaults: bool,
    pub stable_farming: StableFarming,
    pub vaults: Option<serde_json::Value>,
    pub pure_tvl_in_usd: String,
    #[serde(rename = "vol_in_usd_24h")]
    pub vol_in_usd_24_h: String,
    #[serde(rename = "fee_24_h")]
    pub fee_24__h: String,
    pub total_apr: String,
    pub apr: Apr,
    pub extensions: Option<serde_json::Value>,
    pub has_mining: bool,
    pub has_farming: bool,
    pub no_incentives: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Apr {
    #[serde(rename = "fee_apr_24h")]
    pub fee_apr_24_h: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coin {
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub address: String,
    pub balance: String,
    pub logo_url: String,
    pub coingecko_id: String,
    pub project_url: String,
    pub labels: Vec<Option<serde_json::Value>>,
    pub is_trusted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub coin_a: i64,
    pub coin_b: i64,
    pub tick_spacing: i64,
    pub fee_rate: i64,
    pub liquidity: String,
    pub current_sqrt_price: String,
    pub rewarder_manager: RewarderManager,
    pub is_pause: bool,
    pub index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RewarderManager {
    pub fields: Fields,
}

#[derive(Serialize, Deserialize)]
pub struct Fields {
    pub rewarders: Vec<Option<serde_json::Value>>,
    pub points_released: String,
    pub points_growth_global: String,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct StableFarming {
    pub stable_farming_pool: String,
    pub is_show_rewarder: bool,
    pub show_rewarder_1: bool,
    pub show_rewarder_2: bool,
    pub show_rewarder_3: bool,
    pub tvl: String,
    pub apr: String,
    pub effective_tick_lower: i64,
    pub effective_tick_upper: i64,
    pub total_wrapped_amount: String,
    pub stable_rewarder: Option<serde_json::Value>,
}
