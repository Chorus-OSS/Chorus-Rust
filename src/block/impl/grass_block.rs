use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_permutation::BlockPermutation;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub struct GrassBlock;

impl GrassBlock {
    pub const PERMUTATION: Lazy<BlockPermutation> = Lazy::new(||
        BlockPermutation::create(
            block_id::GRASS_BLOCK,
            Vec::new(),
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