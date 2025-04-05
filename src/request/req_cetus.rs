use reqwest::blocking;
use serde::Deserialize;
use serde_json::Value;
use crate::models::cetus::Cetus;


const API_URL: &str = "https://api-sui.cetus.zone/v2/sui/stats_pools?is_vaults=false&display_all_pools=false&has_mining=true&has_farming=true&no_incentives=true&order_by=-totalApr&limit=1&offset=0&coin_type&pool";

pub fn get_pools_amount() -> i64 {
    let response = reqwest::blocking::get(API_URL).unwrap().text().unwrap();
    let pools: Cetus = serde_json::from_str(&response).unwrap();
    pools.data.total
}
