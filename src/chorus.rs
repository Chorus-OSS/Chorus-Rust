use std::clone::Clone;
use std::env::Args;
use std::ops::Deref;
use log::{error, info};
use tokio::sync::{Mutex, RwLock};
use once_cell::sync::Lazy;
use crate::server::Server;
use crate::utils::sem_version::SemVersion;

pub static CODE_NAME: &str = "Chorus";
pub static API_VERSION: &str = "0.0.1";

pub static GAME_VERSION: &str = "1.21.70";

pub static SEM_VERSION: Lazy<SemVersion> = Lazy::new(|| SemVersion::new(1, 21, 7, 0 ,0));
pub static BLOCK_STATE_VERSION: Lazy<i32> = Lazy::new(|| {
    let v = SEM_VERSION.deref();
    (v.major << 24) | (v.minor << 16) | (v.patch << 8)
});

static TITLE: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(true));
static ANSI: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(true));

pub async fn main(args: Args) {
    if *TITLE.read().await {
        info!("Starting Chorus...")
    }

    Server::get_mut()
        .await
        .start()
        .await
        .unwrap_or_else(|e| {
            error!("{}", e);
        });

    if *TITLE.read().await {
        info!("Stopping Chorus...")
    }
    info!("Stopped.")
}