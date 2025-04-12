use crate::models::turbos::Turbos;
use crate::request::{req_cetus, req_turbos};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Result, Write};
use serde_json;
use chrono::Local;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct PoolInfo {
    source: String,
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

fn write_to_file(pools_collection: &PoolsCollection, timestamp: &str) -> io::Result<()> {
    let mut path = PathBuf::from("data");
    fs::create_dir_all(&path)?;
    path.push(format!("{}.json", timestamp));

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)?;
    let mut buf_writer = BufWriter::new(file);
    serde_json::to_writer(&mut buf_writer, pools_collection)?;
    buf_writer.flush()?;
    Ok(())
}

fn cetus() -> PoolsCollection {
    let mut pools_collection = PoolsCollection { pools: Vec::new() };
    match req_cetus::get_pools() {
        Ok(pools) => {
            for pool in pools.data.lp_list {
                let pool_info = PoolInfo {
                    source: "Cetus".to_string(),
                    symbol: pool.symbol,
                    total_apr: pool.total_apr,
                    liquidity: pool.pure_tvl_in_usd,
                    vol_in_usd_24_h: pool.vol_in_usd_24_h,
                    fee_24_h: pool.fee_24_h,
                };
                pools_collection.pools.push(pool_info);
            }
        }
        Err(e) => eprintln!("获取池数据时出错: {}", e),
    }
    pools_collection
}

fn turbos() -> PoolsCollection {
    let mut pools_collection = PoolsCollection { pools: Vec::new() };
    let pools: Turbos = req_turbos::get_pools();
    for pool in pools.data.list {
        let symbol = format!("{}-{}", pool.coin_symbol_a, pool.coin_symbol_b);
        let pool_info = PoolInfo {
            source: "Turbos".to_string(),
            symbol,
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
    let cetus_pools = cetus();
    let turbos_pools = turbos();
    let pools_collection = PoolsCollection {
        pools: [cetus_pools.pools.clone(), turbos_pools.pools.clone()].concat(),
    };

    let local_time = Local::now();
    let timestamp = local_time.format("%Y-%m-%d_%H-%M-%S_%3f").to_string();

    write_to_file(&pools_collection, &timestamp)?;
    Ok(())
}
