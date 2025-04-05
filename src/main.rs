use log::{debug, error, log_enabled, info, Level};
use serde::de;

mod models;
mod request;

fn main() {
    env_logger::init();
    info!("Starting up...");
}
