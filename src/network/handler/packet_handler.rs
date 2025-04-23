use bedrockrs::proto::{v786, GamePacket};

pub trait PacketHandler : Send {
    fn handle(&self, packet: v786::GamePackets); 
}