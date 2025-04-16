use once_cell::sync::Lazy;
use crate::level::block::block_id;
use crate::level::block::block_properties::BlockProperties;

pub const AIR: Lazy<BlockProperties> = Lazy::new(|| 
    BlockProperties::create(block_id::AIR, vec![], None).unwrap()
);