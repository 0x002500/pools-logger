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
    pub pool_type: PoolType,
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
    pub labels: Option<Vec<Option<serde_json::Value>>>,
    pub coin_a: Coin,
    pub coin_b: Coin,
    pub price: String,
    pub rewarder_usd: Vec<String>,
    pub rewarder_apr: Vec<String>,
    pub is_forward: bool,
    pub price_range_config: Option<serde_json::Value>,
    pub object: Object,
    pub category: String,
    pub is_vaults: bool,
    pub stable_farming: Option<StableFarming>,
    pub vaults: Option<Vec<String>>,
    pub pure_tvl_in_usd: String,
    #[serde(rename = "vol_in_usd_24h")]
    pub vol_in_usd_24_h: String,
    #[serde(rename = "fee_24_h")]
    pub fee_24__h: String,
    pub total_apr: String,
    pub apr: Apr,
    pub extensions: Option<Extensions>,
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
    pub labels: Vec<Label>,
    pub is_trusted: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Label {
    Wormhole,
}

#[derive(Serialize, Deserialize)]
pub struct Extensions {
    pub show_rewarder_4: Option<String>,
    pub show_rewarder_5: Option<String>,
    pub zap: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub coin_a: f64,
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
    pub fields: RewarderManagerFields,
}

#[derive(Serialize, Deserialize)]
pub struct RewarderManagerFields {
    pub rewarders: Vec<Rewarder>,
    pub points_released: String,
    pub points_growth_global: String,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Rewarder {
    pub fields: RewarderFields,
}

#[derive(Serialize, Deserialize)]
pub struct RewarderFields {
    pub reward_coin: RewardCoin,
    pub emissions_per_second: String,
    pub growth_global: String,
}

#[derive(Serialize, Deserialize)]
pub struct RewardCoin {
    pub fields: RewardCoinFields,
}

#[derive(Serialize, Deserialize)]
pub struct RewardCoinFields {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PoolType {
    #[serde(rename = "")]
    Empty,
    Unstable,
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
    pub total_wrapped_amount: TotalWrappedAmount,
    pub stable_rewarder: Option<Vec<StableRewarder>>,
}

#[derive(Serialize, Deserialize)]
pub struct StableRewarder {
    pub apr: String,
    pub coin: String,
    pub amount_second: String,
    pub amount_usd: String,
    pub symbol: String,
    pub name: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Serialize, Deserialize)]
pub enum TotalWrappedAmount {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "1693687749723846")]
    The1693687749723846,
}
