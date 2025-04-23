use bedrockrs::proto::{v786, GamePacket};

pub trait PacketHandler {
    fn handle(packet: v786::GamePackets); 
}