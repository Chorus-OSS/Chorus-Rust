use crate::block::block::TBlock;
use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_state::BlockState;
use crate::block::block_states::BlockStates;
use crate::level::level::Level;
use once_cell::sync::Lazy;
use vek::Vec3;

static STATES: Lazy<BlockStates> = Lazy::new(||
    BlockStates::create(block_id::AIR, vec![]).unwrap()
);

static DEFAULT_STATE: Lazy<BlockState> = Lazy::new(||
    STATES.get_default_state()
);

static PROPERTIES: Lazy<BlockAttributes> = Lazy::new(|| {
    let mut attributes = BlockAttributes::default(block_id::AIR);
    attributes.is_solid = false;
    attributes.is_transparent = true;
    attributes.hardness = 0.0;
    attributes.resistance = 0.0;
    attributes
});

pub struct Air {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}

impl TBlock for Air {
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
        &PROPERTIES
    }
}