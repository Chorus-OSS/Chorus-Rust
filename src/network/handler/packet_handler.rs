use crate::network::handler::{login_packet_handler, start_session_handler};
use crate::network::protocol::GamePackets;
use crate::network::session::Session;

#[derive(Clone, Debug)]
pub enum PacketHandler {
    None,
    StartSession,
    LoginPacket,
}

impl PacketHandler {
    pub async fn handle(&self, session: &mut Session, packet: GamePackets) {
        match self {
            PacketHandler::None => {}
            PacketHandler::StartSession => start_session_handler::handle(session, packet).await,
            PacketHandler::LoginPacket => login_packet_handler::handle(session, packet).await,
        }
    }
}