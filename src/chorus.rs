use std::env::Args;
use log::{error, info};
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use crate::server::Server;

static CODE_NAME: &str = "Chorus";
static API_VERSION: &str = "0.0.1";
static TITLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));
static ANSI: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

pub async fn main(args: Args) {
    if *TITLE.lock().await {
        info!("Starting Chorus...")
    }

    Server::get_mut()
        .await
        .start()
        .await
        .unwrap_or_else(|e| {
            error!("{}", e);
        });

    if *TITLE.lock().await {
        info!("Stopping Chorus...")
    }
    info!("Stopped.")
}