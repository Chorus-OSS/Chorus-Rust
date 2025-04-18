use crate::block::property::block_property_type::BlockProperty;
use std::fmt::Display;

#[derive(Debug)]
pub struct BlockStatesCreateError {
    pub identifier: String,
    pub properties: Vec<BlockProperty>
}

impl Display for BlockStatesCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BlockStateInitError {{ identifier: {:?}, properties: {:?} }}", self.identifier, self.properties)
    }
}

impl std::error::Error for BlockStatesCreateError {}