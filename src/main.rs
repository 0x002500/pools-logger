use env_logger::{Builder, Env};
use log::{info, LevelFilter};
use std::io::Write;
use chrono::Local;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

fn init_logger() {
    Builder::from_env(Env::default())
        .format(|buf: &mut env_logger::fmt::Formatter, record: &log::Record<'_>| {
            let local_time: chrono::DateTime<Local> = Local::now();
            let timestamp: String = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

            let level: log::Level = record.level();
            let args: &std::fmt::Arguments<'_> = record.args();

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
    let mut scheduler: Scheduler = Scheduler::new();

    scheduler.every(5.minutes()).run(|| {
        if let Err(e) = job::job() {
            log::error!("Job execution failed: {}", e);
        }
    });

    info!("Starting the application...");
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10000));
    }
}
