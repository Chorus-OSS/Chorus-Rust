use crate::block::block::Block;
use crate::level::block::block_id;
use crate::level::block::block_properties::BlockProperties;
use crate::level::block::block_state::BlockState;
use once_cell::sync::Lazy;
use vek::Vec3;

pub struct Air {
    state: BlockState,
    position: Vec3<i32>,
    level: todo!("Level"),
}

impl Air {
    pub const PROPERTIES: Lazy<BlockProperties> = Lazy::new(||
        BlockProperties::create(block_id::AIR, vec![], None).unwrap()
    );
    
    pub const DEFAULT_STATE: Lazy<BlockState> = Lazy::new(||
        Self::PROPERTIES.get_default_state()
    );
}

impl Block for Air {
    fn get_state(&self) -> &BlockState {
        &self.state
    }

    fn get_position(&self) -> &Vec3<i32> {
        &self.position
    }

    fn get_level(&self) -> &todo!("Level") {
        &self.level
    }

    fn get_properties() -> BlockProperties {
        Self::PROPERTIES.clone()
    }

    fn get_default_state() -> BlockState {
        Self::DEFAULT_STATE.clone()
    }
}