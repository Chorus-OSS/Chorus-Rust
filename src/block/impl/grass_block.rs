use crate::block::block::TBlock;
use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_state::BlockState;
use crate::block::block_states::BlockStates;
use crate::level::level::Level;
use once_cell::sync::Lazy;
use vek::Vec3;

static STATES: Lazy<BlockStates> = Lazy::new(||
    BlockStates::create(block_id::GRASS_BLOCK, vec![]).unwrap()
);

static DEFAULT_STATE: Lazy<BlockState> = Lazy::new(||
    STATES.get_default_state()
);

static ATTRIBUTES: Lazy<BlockAttributes> = Lazy::new(|| {
    let mut attributes = BlockAttributes::default(block_id::GRASS_BLOCK);
    attributes.is_solid = true;
    attributes.is_transparent = false;
    attributes.resistance = 0.6;
    attributes.hardness = 0.6;
    attributes
});

pub struct GrassBlock {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}

impl TBlock for GrassBlock {
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
        &STATES
    }

    fn get_default_state() ->  &'static BlockState {
        &DEFAULT_STATE
    }

    fn get_attributes() -> &'static BlockAttributes { 
        &ATTRIBUTES 
    }
}