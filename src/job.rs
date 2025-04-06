use crate::models::{cetus::Cetus, turbos::Turbos};
use crate::request::{req_cetus, req_turbos};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Result};
use serde_json;
use chrono::Local;

#[derive(Serialize, Deserialize, Clone)]
struct PoolInfo {
    symbol: String,
    total_apr: String,
    liquidity: String,
    vol_in_usd_24_h: String,
    fee_24_h: String,
}

#[derive(Serialize, Deserialize)]
struct PoolsCollection {
    pools: Vec<PoolInfo>,
}

fn write_to_file(pools_collection: &PoolsCollection, path: &str) -> Result<()> {
    let file = File::create(path)?;
    let buf_writer = BufWriter::with_capacity(4 * 1024 * 1024, file);

    serde_json::to_writer(buf_writer, pools_collection)?;

    Ok(())
}

fn cetus() -> PoolsCollection {
    let mut pools_collection: PoolsCollection = PoolsCollection { pools: Vec::new() };
    let pools: Cetus = req_cetus::get_pools();
    for pool in pools.data.lp_list {
        let pool_info: PoolInfo = PoolInfo {
            symbol: pool.symbol,
            total_apr: pool.total_apr,
            liquidity: pool.pure_tvl_in_usd,
            vol_in_usd_24_h: pool.vol_in_usd_24_h,
            fee_24_h: pool.fee_24__h,
        };

        pools_collection.pools.push(pool_info);
    }
    pools_collection
}

fn turbos() -> PoolsCollection {
    let mut pools_collection: PoolsCollection = PoolsCollection { pools: Vec::new() };
    let pools: Turbos = req_turbos::get_pools();
    for pool in pools.data.list {
        let symbol: String = format!("{}-{}", pool.coin_symbol_a, pool.coin_symbol_b);
        let pool_info: PoolInfo = PoolInfo {
            symbol: symbol,
            total_apr: pool.apr.to_string(),
            liquidity: pool.liquidity_usd.to_string(),
            vol_in_usd_24_h: pool.volume_24_h_usd.to_string(),
            fee_24_h: pool.fee_24_h_usd.to_string(),
        };

        pools_collection.pools.push(pool_info);
    }
    pools_collection
}

pub fn job() -> Result<()> {
    let cetus_pools: PoolsCollection = cetus();
    let turbos_pools: PoolsCollection = turbos();
    let pools_collection: PoolsCollection = PoolsCollection {
        pools: [cetus_pools.pools.clone(), turbos_pools.pools.clone()].concat(),
    };

    let local_time = Local::now();
    let timestamp = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let path: String = format!("./data/{}.json", timestamp);
    write_to_file(&pools_collection, &path)?;

    Ok(())
}
