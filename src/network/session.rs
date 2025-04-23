use std::error::Error;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::connection::shard::arc::{shard, ConnectionShared};
use bedrockrs::proto::{v786, ProtoHelper};
use log::info;

pub struct Session {
    connection_shard: ConnectionShared<v786::ProtoHelperV786>
}

impl Session {
    pub fn new(conn: Connection) -> Self {
        let shard = shard::<v786::ProtoHelperV786>(conn);
        
        Self {
            connection_shard: shard
        }
    }
    
    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection_shard.recv().await?;
        
        if let Some(packet) = self.connection_shard.read().await {
            info!("Packet: {:?}", packet);
        }
        
        Ok(())
    }
}