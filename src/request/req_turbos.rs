use log::info;
use crate::models::turbos::Turbos;

fn get_pools_amount() -> i64 {
    let api_url: &str = "https://api.turbos.finance/pools/v2?page=1&pageSize=5&orderBy&category&coinTypes&includeRisk=false&symbol";
    info!("Getting the number of pools from Turbos Finance API");
    let response: String = reqwest::blocking::get(api_url).unwrap().text().unwrap();
    let pools: Turbos = serde_json::from_str(&response).unwrap();
    pools.data.total
}

pub fn get_pools() -> Turbos {
    let api_url: String = format!("https://api.turbos.finance/pools/v2?page=1&pageSize={}&orderBy&category&coinTypes&includeRisk=false&symbol", get_pools_amount());
    info!("Getting the pools data from Turbos Finance API");
    let response: String = reqwest::blocking::get(api_url).unwrap().text().unwrap();
    let pools: Turbos = serde_json::from_str(&response).unwrap();
    pools
}
