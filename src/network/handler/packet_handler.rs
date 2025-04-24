use crate::network::protocol::GamePackets;

pub trait PacketHandler : Send + Sync + 'static {
    fn handle(&mut self, packet: GamePackets); 
}