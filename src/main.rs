use std::process::exit;
use log::{error, info};

mod chorus;
mod logger;
mod config;
mod server;
mod utils;
mod network;
mod level;

fn main() {
    let config = config::setup_config();
    
    logger::setup_logger(config.log_to_file, &config.logs_directory);
    
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .unwrap_or_else(|err| {
            error!("Failed to create Tokio-Runtime, Err: {err:?}");
            exit(1)
        });
    
    runtime.block_on(async {
        chorus::main(std::env::args()).await
    })
}