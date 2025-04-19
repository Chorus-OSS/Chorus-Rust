use crate::block::state::block_state_type::BlockStateType;
use std::fmt::Display;

#[derive(Debug)]
pub struct BlockPermutationCreateError {
    pub identifier: String,
    pub states: Vec<BlockStateType>
}

impl Display for BlockPermutationCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BlockPermutationCreateError {{ identifier: {:?}, states: {:?} }}", self.identifier, self.states)
    }
}

impl std::error::Error for BlockPermutationCreateError {}