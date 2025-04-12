use crate::models::turbos::Turbos;
use log::{error, info};
use reqwest::blocking::Client;
use std::error::Error;

fn get_pools_amount() -> Result<i64, Box<dyn Error>> {
    let api_url: &str = "https://api.turbos.finance/pools/v2?page=1&pageSize=5&orderBy&category&coinTypes&includeRisk=false&symbol";
    info!("Getting the number of pools from Turbos Finance API");

    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(api_url).send()?;

    if !response.status().is_success() {
        error!("Error handel, Status Code: {}", response.status());
        return Err(format!("Error handel, Status Code: {}", response.status()).into());
    }

    let response_text = response.text()?;
    let pools: Turbos = serde_json::from_str(&response_text)?;

    Ok(pools.data.total)
}

pub fn get_pools() -> Result<Turbos, Box<dyn Error>> {
    let total_pools = get_pools_amount()?;
    let api_url: String = format!("https://api.turbos.finance/pools/v2?page=1&pageSize={}&orderBy&category&coinTypes&includeRisk=false&symbol", total_pools);
    info!("Getting the pools data from Turbos Finance API");

    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(&api_url).send()?;

    if !response.status().is_success() {
        error!("Error handel, Status Code: {}", response.status());
        return Err(format!("Error handel, Status Code: {}", response.status()).into());
    }

    let response_text: String = response.text()?;
    let pools: Turbos = serde_json::from_str(&response_text)?;

    Ok(pools)
}
