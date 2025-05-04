use log::info;
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::protocol;
use crate::network::protocol::enums::{PacketCompressionAlgorithm, PlayStatus};
use crate::network::protocol::GamePackets;
use crate::network::protocol::packets::NetworkSettingsPacket;
use crate::network::session::Session;
use crate::network::session::state::SessionState;

pub async fn handle(session: &mut Session, packet: GamePackets) {
    let GamePackets::RequestNetworkSettings(packet) = packet else { return; };

    let protocol = packet.client_network_version;

    if protocol != protocol::info::PROTOCOL_VERSION {
        session.send_play_status(
            if protocol < protocol::info::PROTOCOL_VERSION {
                PlayStatus::LoginFailedClientOld
            } else {
                PlayStatus::LoginFailedServerOld
            },
            true
        ).await;

        session.close(
            if protocol < protocol::info::PROTOCOL_VERSION {
                Some("disconnectionScreen.outdatedClient")
            } else {
                Some("disconnectionScreen.outdatedServer")
            }
        ).await;
    }

    // TODO: IP Bans
    let mut conn = session.get_mut_connection_shard();
    conn.write(
        GamePackets::NetworkSettings(
            NetworkSettingsPacket {
                compression_threshold: 1,
                compression_algorithm: PacketCompressionAlgorithm::None,
                client_throttle_enabled: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }
        )
    ).await.unwrap();
    
    session.packet_handler = PacketHandler::LoginPacket;
    session.get_mut_state().handle(&SessionState::Login).await;
}