pub mod state;

use std::error::Error;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::connection::shard::arc::{shard, ConnectionShared};
use bedrockrs::proto::{v786, GamePacketsAll, ProtoHelper};
use log::info;
use crate::network::handler::packet_handler::PacketHandler;

pub struct Session {
    connection_shard: ConnectionShared<v786::ProtoHelperV786>,
    pub packet_handler: Option<Box<dyn PacketHandler>>,
}

impl Session {
    pub fn new(conn: Connection) -> Self {
        let shard = shard::<v786::ProtoHelperV786>(conn);

        Self {
            connection_shard: shard,
            packet_handler: None,
        }
    }

    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection_shard.send().await?;
        self.connection_shard.recv().await?;

        while let Some(packet) = self.connection_shard.read().await {
            info!("Packet: {:?}", packet.id());
        }

        Ok(())
    }
    
    pub async fn on_login_success(&mut self) {
        self.connection_shard.write(v786::GamePackets::PlaySatus(v786::packets::PlayStatusPacket {
            status: v786::enums::PlayStatus::LoginSuccess
        })).await.unwrap();
    }
}