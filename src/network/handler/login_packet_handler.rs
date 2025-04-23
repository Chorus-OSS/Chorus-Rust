use bedrockrs::proto::v786::GamePackets;
use crate::network::handler::packet_handler::PacketHandler;

pub struct LoginPacketHandler;

impl PacketHandler for LoginPacketHandler {
    fn handle(packet: GamePackets) {
        let GamePackets::Login(packet) = packet else { return; };
        
    }
}