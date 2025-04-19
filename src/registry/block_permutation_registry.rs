use crate::block::block_permutation::BlockPermutation;
use crate::error::registry::already_registered::AlreadyRegisteredError;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Deref;
use tokio::sync::RwLock;
use crate::block::block_id;
use crate::block::r#impl::air;
use crate::block::r#impl::air::Air;
use crate::block::r#impl::bedrock::Bedrock;
use crate::block::r#impl::dirt::Dirt;
use crate::block::r#impl::grass_block::GrassBlock;

static INSTANCE: Lazy<RwLock<BlockPermutationRegistry>> = Lazy::new(|| {
    RwLock::new(
        BlockPermutationRegistry::create()
    )
});

#[non_exhaustive]
pub struct BlockPermutationRegistry {
    registry: HashMap<String, Lazy<BlockPermutation>>,
}

impl BlockPermutationRegistry {
    fn create() -> Self {
        let mut val = Self { registry: HashMap::new() };

        val.register(block_id::AIR, Lazy::new(|| Air::PERMUTATION.clone() )).unwrap();
        val.register(block_id::DIRT, Lazy::new(|| Dirt::PERMUTATION.clone() )).unwrap();
        val.register(block_id::GRASS_BLOCK, Lazy::new(|| GrassBlock::PERMUTATION.clone() )).unwrap();
        val.register(block_id::BEDROCK, Lazy::new(|| Bedrock::PERMUTATION.clone() )).unwrap();

        val
    }

    pub fn instance() -> &'static RwLock<BlockPermutationRegistry> {
        &INSTANCE
    }

    pub fn register(&mut self, identifier: &str, block_states: Lazy<BlockPermutation>) -> Result<(), AlreadyRegisteredError> {
        if self.registry.contains_key(identifier) {
            Err(AlreadyRegisteredError { identifier: identifier.to_string() })
        } else {
            self.registry.insert(identifier.to_string(), block_states);
            Ok(())
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&BlockPermutation> {
        self.registry.get(identifier).map(|v| v.deref())
    }
}