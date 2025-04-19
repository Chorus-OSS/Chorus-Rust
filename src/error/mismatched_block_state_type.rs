use std::fmt::{Display, Formatter};
use crate::block::state::block_state_type::BlockStateType;

#[derive(Debug)]
pub struct MismatchedBlockStateTypeError {
    pub found: BlockStateType,
    pub expected: String,
}

impl Display for MismatchedBlockStateTypeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MismatchedBlockStateTypeError: {{ found: {:?}, expected: {:?} }}", self.found, self.expected)
    }
}

impl std::error::Error for MismatchedBlockStateTypeError {}