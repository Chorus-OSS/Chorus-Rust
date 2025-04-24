use std::sync::Weak;
use statig::prelude::*;
use tokio::sync::{Mutex, MutexGuard};
use crate::network::protocol::GamePackets;
use crate::network::protocol;
use crate::network::protocol::packets::NetworkSettingsPacket;
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::protocol::enums::{PacketCompressionAlgorithm, PlayStatus};
use crate::network::session::{Session};
use crate::network::session::state::SessionState;

pub struct StartSessionHandler {
    pub session: Weak<Mutex<Session>>,   
}

impl StartSessionHandler {
    pub fn new(session: Weak<Mutex<Session>>) -> Self {
        Self { session }
    }
}

impl PacketHandler for StartSessionHandler {
    fn handle(&mut self, packet: GamePackets) {
        let GamePackets::RequestNetworkSettings(packet) = packet else { return; };
        
        tokio::spawn({
            let session = self.session.clone();
            async move {
                let Some(mut session) = session.upgrade() else { return; };
                let mut session = session.lock().await;

                let protocol = packet.client_network_version;

                if protocol != protocol::info::PROTOCOL_VERSION {
                    session.send_play_status(
                        if protocol < protocol::info::PROTOCOL_VERSION {
                            PlayStatus::LoginFailedClientOld
                        } else {
                            PlayStatus::LoginFailedServerOld
                        }
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

                session.get_mut_connection_shard().write(
                    GamePackets::NetworkSettings(
                        NetworkSettingsPacket {
                            compression_threshold: 1,
                            compression_algorithm: PacketCompressionAlgorithm::ZLib,
                            client_throttle_enabled: false,
                            client_throttle_threshold: 0,
                            client_throttle_scalar: 0.0,
                        }
                    )
                ).await.unwrap();

                session.get_mut_state().handle(&SessionState::Login).await;
            }
        });
    }
}

