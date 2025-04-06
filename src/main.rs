use env_logger::{Builder, Env};
use log::{info, LevelFilter};
use std::io::Write;
use chrono::Local;

fn init_logger() {
    Builder::from_env(Env::default())
        .format(|buf, record| {
            let local_time = Local::now();
            let timestamp = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

            let level = record.level();
            let args = record.args();

            // 根据级别使用不同的颜色
            match level {
                log::Level::Error => writeln!(buf, "\x1B[31m[{}] ERROR: {}\x1B[0m", timestamp, args),
                log::Level::Warn => writeln!(buf, "\x1B[33m[{}] WARN: {}\x1B[0m", timestamp, args),
                log::Level::Info => writeln!(buf, "\x1B[32m[{}] INFO: {}\x1B[0m", timestamp, args),
                log::Level::Debug => writeln!(buf, "\x1B[34m[{}] DEBUG: {}\x1B[0m", timestamp, args),
                log::Level::Trace => writeln!(buf, "\x1B[90m[{}] TRACE: {}\x1B[0m", timestamp, args),
            }
        })
        .filter_level(LevelFilter::Info)
        .init();
}

mod request;
mod models;
mod job;

fn main() {
    init_logger();
    info!("Starting the application...");
}
