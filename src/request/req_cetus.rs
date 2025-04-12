use log::{error, info};
use crate::models::cetus::Cetus;
use reqwest::blocking::Client;
use std::error::Error;

fn get_pools_amount() -> Result<i64, Box<dyn Error>> {
    let api_url = "https://api-sui.cetus.zone/v2/sui/stats_pools?is_vaults=false&display_all_pools=false&has_mining=true&has_farming=true&no_incentives=true&order_by=-totalApr&limit=1&offset=0&coin_type&pool";
    info!("Getting the number of pools from Cetus's API");

    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(api_url).send()?;

    if !response.status().is_success() {
        error!("Error handel, Status Code: {}", response.status());
        return Err(format!("Error handel, Status Code: {}", response.status()).into());
    }

    let response_text = response.text()?;
    let pools: Cetus = serde_json::from_str(&response_text)?;

    Ok(pools.data.total)
}

pub fn get_pools() -> Result<Cetus, Box<dyn Error>> {
    let total_pools = get_pools_amount()?;
    let api_url = format!(
        "https://api-sui.cetus.zone/v2/sui/stats_pools?is_vaults=false&display_all_pools=false&has_mining=true&has_farming=true&no_incentives=true&order_by=-totalApr&limit={}&offset=0&coin_type&pool",
        total_pools
    );
    info!("Getting data from Cetus's API");

    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.get(&api_url).send()?;

    if !response.status().is_success() {
        error!("Error handel, Status Code: {}", response.status());
        return Err(format!("Error handel, Status Code: {}", response.status()).into());
    }

    let response_text: String = response.text()?;
    let pools: Cetus = serde_json::from_str(&response_text)?;

    Ok(pools)
}
