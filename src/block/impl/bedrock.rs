use std::collections::HashSet;
use once_cell::sync::Lazy;
use crate::block::block_attributes::BlockAttributes;
use crate::block::block_id;
use crate::block::block_permutation::BlockPermutation;

pub struct Bedrock;

impl Bedrock {
    pub(crate) const PERMUTATION: Lazy<BlockPermutation> = Lazy::new(||
        BlockPermutation::create(
            block_id::BEDROCK,
            Vec::new(),
            BlockAttributes {
                tags: HashSet::new(),
                friction_factor: 0.6,
                is_solid: true,
                is_transparent: false,
                hardness: -1.0,
                resistance: 3_600_000.0,
                burn_chance: 0,
                burn_ability: 0
            }
        ).expect("Vanilla")
    );
}