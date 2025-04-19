use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_type::BlockType;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::block::component::block_components::BlockComponents;

pub struct GrassBlock;

impl GrassBlock {
    pub const TYPE: Lazy<BlockType> = Lazy::new(||
        BlockType::create(
            block_id::GRASS_BLOCK,
            Vec::new(),
            BlockComponents::create(vec![]),
            BlockAttributes {
                tags: HashSet::new(),
                friction_factor: 0.6,
                is_solid: true,
                is_transparent: false,
                hardness: 0.6,
                resistance: 0.6,
                burn_chance: 0,
                burn_ability: 0
            }
        ).expect("Vanilla")
    );
}