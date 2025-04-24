use std::collections::HashMap;
use std::error::Error;
use std::io::{Cursor, Read};
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::protocol::GamePackets;
use crate::network::session::Session;
use std::sync::Weak;
use bedrockrs::proto::ProtoCodecLE;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::server::Server;

pub struct LoginPacketHandler {
    pub session: Weak<Mutex<Session>>,
}

pub struct ChainData {
    issue_time: i64,
    username: String,
    client_uuid: Uuid,
    title_id: String,
}

impl LoginPacketHandler {
    pub fn new(session: Weak<Mutex<Session>>) -> Self {
        Self { session }
    }
    
    fn decode_chain_data(stream: &mut Cursor<&[u8]>) -> Option<ChainData> {
        let length = <i32 as ProtoCodecLE>::proto_deserialize(stream).ok()?;
        
        let mut chain_buffer = Vec::<u8>::with_capacity(length as usize);
        stream.take(length as u64).read_to_end(&mut chain_buffer).ok()?;
        
        let chain_json = String::from_utf8(chain_buffer).ok()?;
        let map = serde_json::from_str::<HashMap<String, Vec<String>>>(&chain_json).ok()?;
        
        if let Some(chains) = map.get("chain") {
            for chain in chains {
                
            }
            
            None
        } else { None }
    }
}

impl PacketHandler for LoginPacketHandler {
    fn handle(&mut self, packet: GamePackets) {
        let GamePackets::Login(packet) = packet else { return; };
        
        tokio::spawn({
            let session = self.session.clone();
            async move {
                let server = Server::get().await;
                
                let req_bytes = Cursor::new(packet.connection_request.as_bytes());
            }
        });
    }
}