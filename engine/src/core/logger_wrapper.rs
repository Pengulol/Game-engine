// logger

use log::{LevelFilter, Log, Metadata, Record};
use env_logger::Builder;

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}


pub fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
   }

pub fn log_info(message: &str) {
    log::info!("{}", message);
}


pub fn log_warn(message: &str) {
    log::warn!("{}", message);
}

pub fn log_error(message: &str) {
    log::error!("{}", message);
}