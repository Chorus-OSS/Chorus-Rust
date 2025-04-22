use crate::chorus;
use crate::config::server_properties::ServerProperties;
use crate::server::Server;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::error::ListenerError;
use bedrockrs::proto::listener::Listener;
use bedrockrs::proto::v786;
use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Network {
    listener: Arc<Mutex<Listener>>,
}

impl Network {
    pub async fn default(properties: &ServerProperties) -> Self {
        Self {
            listener: Arc::new(Mutex::new(
                Listener::new_raknet(
                    properties.motd.clone(),
                    properties.sub_motd.clone(),
                    String::from(chorus::GAME_VERSION),
                    v786::info::PROTOCOL_VERSION,
                    properties.max_players.clone(),
                    0,
                    SocketAddr::new(
                        IpAddr::V4(
                            Ipv4Addr::from_str(properties.server_ip.as_str()).unwrap_or_else(
                                |err| {
                                    error!("{}: {}", err, properties.server_ip);

                                    Ipv4Addr::UNSPECIFIED
                                },
                            ),
                        ),
                        properties.server_port.clone(),
                    ),
                    false,
                )
                .await
                .unwrap(),
            )),
        }
    }

    pub async fn start(&mut self) -> Result<(), ListenerError> {
        self.listener.lock().await.start().await?;

        tokio::spawn({
            let mut listener = self.listener.clone();
            async move {
                loop {
                    let conn = listener.lock().await.accept().await.unwrap();

                    info!("Connected: {}", conn.get_socket_addr().ip().to_string());
                }
            }
        });

        Ok(())
    }
}
