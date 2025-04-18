use std::fmt::{Display, Formatter};
use crate::block::property::block_property_type::BlockProperty;

#[derive(Debug)]
pub struct MismatchedBlockPropertyError {
    pub found: BlockProperty,
    pub expected: String,
}

impl Display for MismatchedBlockPropertyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MismatchedBlockPropertyError: {{ found: {:?}, expected: {:?} }}", self.found, self.expected)
    }
}

impl std::error::Error for MismatchedBlockPropertyError {}