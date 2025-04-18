use crate::level::block::property::r#type::block_property_type::BlockPropertyType;
use std::fmt::Display;

#[derive(Debug)]
pub struct BlockStatesCreateError {
    pub identifier: String,
    pub properties: Vec<BlockPropertyType>
}

impl Display for BlockStatesCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BlockStateInitError {{ identifier: {:?}, properties: {:?} }}", self.identifier, self.properties)
    }
}

impl std::error::Error for BlockStatesCreateError {}