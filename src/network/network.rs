use crate::server::Server;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::error::ListenerError;
use bedrockrs::proto::listener::Listener;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

struct Network {
    listener: Listener
}

impl Default for Network {
    async fn default() -> Self {
        let server = Server::get().await;
        Self {
            listener: Listener::new_raknet(
                server.properties.motd.clone(),
                server.properties.sub_motd.clone(),
                "1.21.60".to_string(),
                server.properties.max_players.clone(),
                0,
                SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::from_str(server.properties.server_ip.as_str()).unwrap(),
                    server.properties.server_port,
                )),
                false,
            ).await.unwrap()
        }
    }
}

impl Network {
    async fn start(&mut self) -> Result<(), ListenerError> {
        self.listener.start().await
    }
    async fn update(&mut self) {
        let conn: Connection = self.listener.accept().await.unwrap();
        
        // tokio::spawn(|| async {});
    }
}