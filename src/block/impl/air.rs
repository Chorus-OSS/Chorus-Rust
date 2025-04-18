use std::collections::HashSet;
use crate::block::block::TBlock;
use crate::level::block::block_id;
use crate::level::block::block_states::BlockStates;
use crate::level::block::block_state::BlockState;
use once_cell::sync::Lazy;
use vek::Vec3;
use crate::block::transparent::block_transparent::TBlockTransparent;
use crate::level::block::block_properties::{BlockProperties, TBlockProperties};
use crate::level::level::Level;

pub struct Air {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}

impl Air {
    pub const STATES: Lazy<BlockStates> = Lazy::new(||
        BlockStates::create(block_id::AIR, vec![]).unwrap()
    );
    
    pub const DEFAULT_STATE: Lazy<BlockState> = Lazy::new(||
        Self::STATES.get_default_state()
    );
    
    pub const PROPERTIES: Lazy<BlockProperties> = Lazy::new(||
        BlockProperties::new(block_id::AIR, None)
    );
}

impl TBlockTransparent for Air {
    fn get_state(&self) -> &BlockState {
        &self.state
    }

    fn get_position(&self) -> &Vec3<i32> {
        &self.position
    }

    fn get_layer(&self) -> &i32 { 
        &self.layer
    }

    fn get_level(&self) -> &Level {
        &self.level
    }

    fn get_states() -> BlockStates {
        Self::STATES.clone()
    }

    fn get_default_state() -> BlockState {
        Self::DEFAULT_STATE.clone()
    }

    fn get_properties() -> Box<dyn TBlockProperties> {
        Box::new(Self::PROPERTIES.clone())
    }
}