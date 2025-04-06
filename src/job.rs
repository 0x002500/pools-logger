use crate::models::{cetus::Cetus, turbos::Turbos};
use crate::request::{req_cetus, req_turbos};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Result};
use serde_json;

#[derive(Serialize, Deserialize)]
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

fn turbos() {

}
