use crate::chorus;
use crate::config::server_properties;
use crate::config::server_properties::ServerProperties;
use crate::network::network::Network;
use crate::utils::rolling_float_average::RollingFloatAverage;
use bedrockrs::proto::listener::Listener;
use chrono::Utc;
use log::{error, info};
use once_cell::sync::Lazy;
use std::cmp::{max, min};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time;
use tokio::time::{Duration, Instant, sleep};

static INSTANCE: OnceCell<RwLock<Server>> = OnceCell::const_new();

pub struct Server {
    pub properties: ServerProperties,
    network: Network,

    is_running: bool,

    tick: i64,
    next_tick_ms: i64,

    tick_min: f64,
    usage_max: f64,

    tick_avg: RollingFloatAverage,
    usage_avg: RollingFloatAverage,
}

impl Server {
    async fn default() -> Self {
        let properties = server_properties::setup_properties();

        Self {
            properties: properties.clone(),
            network: Network::default(&properties).await,

            is_running: true,

            tick: 0,
            next_tick_ms: Utc::now().timestamp_millis(),

            tick_min: 20.0,
            usage_max: 0.0,

            tick_avg: RollingFloatAverage::new(20),
            usage_avg: RollingFloatAverage::new(20),
        }
    }

    pub async fn get() -> RwLockReadGuard<'static, Server> {
        INSTANCE
            .get_or_init(|| async { RwLock::new(Self::default().await) })
            .await
            .read()
            .await
    }

    pub async fn get_mut() -> RwLockWriteGuard<'static, Server> {
        INSTANCE
            .get_or_init(|| async { RwLock::new(Self::default().await) })
            .await
            .write()
            .await
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.network.start().await?;

        info!(
            "Started on {}:{}.",
            self.properties.server_ip, self.properties.server_port
        );

        while self.is_running {
            if let Err(err) = self.tick().await {
                error!("{}", err);
                return Ok(());
            }

            let next_ms = self.next_tick_ms * 1000;
            let current_ms = Utc::now().timestamp_micros();

            if next_ms - 100 > current_ms {
                let allocated = next_ms - current_ms - 1000;
                if allocated > 0 {
                    sleep(Duration::from_micros(allocated as u64)).await
                }
            }
        }

        Ok(())
    }

    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        let tick_start = Utc::now().timestamp_millis();

        let tick_start_nano = Instant::now();

        self.tick += 1;
        
        self.network.tick().await?;

        let tick_elapsed_nano = tick_start_nano.elapsed().as_nanos();
        let tick = f64::min(
            20.0,
            1_000_000_000.0 / f64::max(1_000_000.0, tick_elapsed_nano as f64),
        );
        let usage = f64::min(1.0, tick_elapsed_nano as f64 / 50_000_000.0);

        if self.usage_max < usage {
            self.usage_max = usage;
        }

        if self.tick_min > tick {
            self.tick_min = tick;
        }

        self.tick_avg.add(tick);
        self.usage_avg.add(usage);

        if (self.next_tick_ms - tick_start) < -1000 {
            self.next_tick_ms = tick_start
        } else {
            self.next_tick_ms += 50
        }

        // if self.tick % 20 == 0 {
        //     info!("T: {}, TM: {:.2}, UM: {:.2}, TA: {:.2}, UA: {:.2}", self.tick, self.tick_min, self.usage_max, self.tick_avg.get_avg(), self.usage_avg.get_avg());
        // }

        Ok(())
    }
}
