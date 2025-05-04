pub mod state;

use std::error::Error;
use std::ops::Deref;
use std::sync::{Weak};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::network::protocol::{
    packets::PlayStatusPacket,
    enums::PlayStatus,
    ProtoHelperV786,
    GamePackets,
};
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::connection::shard::arc::{shard, ConnectionShared};
use bedrockrs::proto::{GamePacketsAll, ProtoHelper};
use log::{debug, info};
use statig::awaitable::{IntoStateMachineExt, StateMachine, Super, Transition};
use statig::{state_machine, Response};
use tokio::sync::Mutex;
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::protocol::enums::ConnectionFailReason;
use crate::network::protocol::packets::{DisconnectMessage, DisconnectPacket};
use crate::network::session::state::SessionStateMachine;
use crate::server::Server;

pub struct Session {
    connection_shard: ConnectionShared<ProtoHelperV786>,
    pub packet_handler: PacketHandler,
    closed: AtomicBool,
    state: StateMachine<SessionStateMachine>
}

impl Session {
    pub fn new(conn: Connection) -> Self {
        Self {
            connection_shard: shard::<ProtoHelperV786>(conn),
            packet_handler: PacketHandler::StartSession,
            closed: AtomicBool::new(false),
            state: SessionStateMachine::new().state_machine(),
        }
    }

    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection_shard.send().await?;
        self.connection_shard.recv().await?;

        while let Some(packet) = self.connection_shard.read().await {
            info!("Packet: {:?}", packet.id());
            
            self.packet_handler.clone().handle(self, packet).await;
        }
        
        Ok(())
    }

    pub async fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess, false).await;
    }

    pub async fn send_play_status(&mut self, status: PlayStatus, immediate: bool) {
        info!("Sending play status: {:?}", status);
        
        self.connection_shard.write(
            GamePackets::PlayStatus(
                PlayStatusPacket {
                    status
                }
            )
        ).await.unwrap();
        
        if (immediate) { self.connection_shard.send().await.unwrap() }
    }

    pub fn get_connection_shard(&self) -> &ConnectionShared<ProtoHelperV786> {
        &self.connection_shard
    }

    pub fn get_mut_connection_shard(&mut self) -> &mut ConnectionShared<ProtoHelperV786> {
        &mut self.connection_shard
    }
    
    pub fn get_state(&self) -> &StateMachine<SessionStateMachine> {
        &self.state
    }
    
    pub fn get_mut_state(&mut self) -> &mut StateMachine<SessionStateMachine> {
        &mut self.state
    }

    pub async fn close(&mut self, reason: Option<&str>) {
        if self.closed.load(Ordering::SeqCst) { return; }

        if let Some(reason) = reason {
            self.connection_shard.write(GamePackets::Disconnect(
                DisconnectPacket {
                    reason: ConnectionFailReason::Disconnected,
                    message: Some(DisconnectMessage {
                        kick_message: reason.to_string(),
                        filtered_message: reason.to_string(),
                    })
                }
            )).await.unwrap();

            self.connection_shard.send().await.unwrap();
        }

        self.connection_shard.close().await;

        self.closed.store(true, Ordering::SeqCst)
    }
}