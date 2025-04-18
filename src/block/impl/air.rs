use crate::block::block::TBlock;
use crate::block::transparent::block_transparent::TBlockTransparent;
use crate::level::block::block_id;
use crate::level::block::block_properties::{BlockProperties, TBlockProperties};
use crate::level::block::block_state::BlockState;
use crate::level::block::block_states::BlockStates;
use crate::level::level::Level;
use once_cell::sync::Lazy;
use std::ops::Deref;
use vek::Vec3;

pub struct Air {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
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

    fn get_states() ->  &'static BlockStates {
        static STATES: Lazy<BlockStates> = Lazy::new(||
            BlockStates::create(block_id::AIR, vec![]).unwrap()
        );
        &STATES
    }

    fn get_default_state() ->  &'static BlockState {
        static DEFAULT_STATE: Lazy<BlockState> = Lazy::new(||
            <Air as TBlockTransparent>::get_states().get_default_state()
        );
        &DEFAULT_STATE
    }

    fn get_properties() -> &'static dyn TBlockProperties {
        static PROPERTIES: Lazy<BlockProperties> = Lazy::new(||
            BlockProperties::new(block_id::AIR, None)
        );
        &*PROPERTIES
    }
}