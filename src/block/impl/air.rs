use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_type::BlockType;
use crate::block::component::block_components::BlockComponents;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub struct Air;

impl Air {
    pub const TYPE: Lazy<BlockType> = Lazy::new(|| {
        BlockType::create(
            block_id::AIR,
            Vec::new(),
            BlockComponents::create(vec![]),
            BlockAttributes {
                tags: HashSet::new(),
                friction_factor: 0.6,
                is_solid: false,
                is_transparent: true,
                hardness: 0.0,
                resistance: 0.0,
                burn_chance: 0,
                burn_ability: 0,
            },
        )
        .expect("Vanilla")
    });
}
