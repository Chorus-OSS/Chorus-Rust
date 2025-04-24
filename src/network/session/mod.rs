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
use crate::network::handler::start_session_handler::StartSessionHandler;
use crate::network::protocol::enums::ConnectionFailReason;
use crate::network::protocol::packets::{DisconnectMessage, DisconnectPacket};
use crate::network::session::state::SessionStateMachine;
use crate::server::Server;

pub struct Session {
    connection_shard: ConnectionShared<ProtoHelperV786>,
    pub packet_handler: Option<Box<dyn PacketHandler>>,
    closed: AtomicBool,
    state: Option<StateMachine<SessionStateMachine>>
}

impl Session {
    pub async fn new(conn: Connection) -> Arc<Mutex<Self>> {
        let shard = shard::<ProtoHelperV786>(conn);

        let mut val = Arc::new(
            Mutex::new(
                Self {
                    connection_shard: shard,
                    packet_handler: None,
                    closed: AtomicBool::new(false),
                    state: None,
                }
            )
        );
        
        let mut locked_val = val.lock().await;
        
        locked_val.packet_handler = Some(
            Box::new(
                StartSessionHandler::new(
                    Arc::downgrade(&val),
                )
            )
        );
        
        locked_val.state = Some(
            SessionStateMachine::new(
                Arc::downgrade(&val),
            ).state_machine()
        );
        
        val.clone()
    }

    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection_shard.send().await?;
        self.connection_shard.recv().await?;

        while let Some(packet) = self.connection_shard.read().await {
            info!("Packet: {:?}", packet.id());
            
            if let Some(packet_handler) = self.packet_handler.as_mut() {
                packet_handler.handle(packet);
            }
        }

        Ok(())
    }

    pub async fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess).await;
    }

    pub async fn send_play_status(&mut self, status: PlayStatus) {
        self.connection_shard.write(
            GamePackets::PlayStatus(
                PlayStatusPacket {
                    status
                }
            )
        ).await.unwrap();
    }

    pub fn get_connection_shard(&self) -> &ConnectionShared<ProtoHelperV786> {
        &self.connection_shard
    }

    pub fn get_mut_connection_shard(&mut self) -> &mut ConnectionShared<ProtoHelperV786> {
        &mut self.connection_shard
    }
    
    pub fn get_state(&self) -> &StateMachine<SessionStateMachine> {
        self.state.as_ref().unwrap()
    }
    
    pub fn get_mut_state(&mut self) -> &mut StateMachine<SessionStateMachine> {
        self.state.as_mut().unwrap()
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