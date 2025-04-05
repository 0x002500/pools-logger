use reqwest::blocking;
use serde::Deserialize;
use serde_json::Value;
use crate::models::turbos::Turbos;

fn get_pools_amount() -> i64 {
    let api_url: &str = "https://api.turbos.finance/pools/v2?page=1&pageSize=5&orderBy&category&coinTypes&includeRisk=false&symbol";
    let response: String = reqwest::blocking::get(api_url).unwrap().text().unwrap();
    let pools: Turbos = serde_json::from_str(&response).unwrap();
    pools.data.total
}
