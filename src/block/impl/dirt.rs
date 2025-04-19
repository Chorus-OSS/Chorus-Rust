use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_type::BlockType;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::block::component::block_components::BlockComponents;

pub struct Dirt;

impl Dirt {
    pub const TYPE: Lazy<BlockType> = Lazy::new(||
        BlockType::create(
            block_id::DIRT,
            Vec::new(),
            BlockComponents::create(vec![]),
            BlockAttributes {
                tags: HashSet::new(),
                friction_factor: 0.6,
                is_solid: true,
                is_transparent: false,
                hardness: 0.5,
                resistance: 0.5,
                burn_chance: 0,
                burn_ability: 0
            }
        ).expect("Vanilla")
    );
}