use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    code: i64,
    msg: String,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    total: i64,
    lp_list: Vec<LpList>,
}

#[derive(Serialize, Deserialize)]
pub struct LpList {
    symbol: String,
    name: String,
    decimals: i64,
    fee: String,
    tick_spacing: String,
    pool_type: String,
    address: String,
    coin_a_address: String,
    coin_b_address: String,
    project_url: String,
    is_display_rewarder: bool,
    is_closed: bool,
    rewarder_display1: bool,
    rewarder_display2: bool,
    rewarder_display3: bool,
    rewarder_display4: bool,
    rewarder_display5: bool,
    labels: Option<serde_json::Value>,
    coin_a: Coin,
    coin_b: Coin,
    price: String,
    rewarder_usd: Vec<Option<serde_json::Value>>,
    rewarder_apr: Vec<RewarderApr>,
    is_forward: bool,
    price_range_config: Option<serde_json::Value>,
    object: Object,
    category: String,
    is_vaults: bool,
    stable_farming: StableFarming,
    vaults: Option<serde_json::Value>,
    pure_tvl_in_usd: String,
    #[serde(rename = "vol_in_usd_24h")]
    vol_in_usd_24_h: String,
    #[serde(rename = "fee_24_h")]
    fee_24__h: String,
    total_apr: String,
    apr: Apr,
    extensions: Option<serde_json::Value>,
    has_mining: bool,
    has_farming: bool,
    no_incentives: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Apr {
    #[serde(rename = "fee_apr_24h")]
    fee_apr_24_h: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coin {
    name: String,
    symbol: String,
    decimals: i64,
    address: String,
    balance: String,
    logo_url: String,
    coingecko_id: String,
    project_url: String,
    labels: Vec<String>,
    is_trusted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    coin_a: i64,
    coin_b: i64,
    tick_spacing: i64,
    fee_rate: i64,
    liquidity: String,
    current_sqrt_price: String,
    rewarder_manager: RewarderManager,
    is_pause: bool,
    index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RewarderManager {
    fields: Fields,
}

#[derive(Serialize, Deserialize)]
pub struct Fields {
    rewarders: Vec<Option<serde_json::Value>>,
    points_released: String,
    points_growth_global: String,
    last_updated_time: i64,
}

#[derive(Serialize, Deserialize)]
pub enum RewarderApr {
    #[serde(rename = "0%")]
    The0,
}

#[derive(Serialize, Deserialize)]
pub struct StableFarming {
    stable_farming_pool: String,
    is_show_rewarder: bool,
    show_rewarder_1: bool,
    show_rewarder_2: bool,
    show_rewarder_3: bool,
    tvl: String,
    apr: String,
    effective_tick_lower: i64,
    effective_tick_upper: i64,
    total_wrapped_amount: String,
    stable_rewarder: Option<serde_json::Value>,
}
