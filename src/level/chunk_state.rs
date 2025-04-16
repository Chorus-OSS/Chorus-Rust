pub enum ChunkState {
    New,
    Generated,
    Populated,
    Finished
}

impl ChunkState {
    pub fn can_send(&self) -> bool {
        match self {
            ChunkState::Finished => true,
            _ => false
        }
    }
}