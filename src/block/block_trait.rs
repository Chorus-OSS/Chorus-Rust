use crate::level::block::block_properties::BlockProperties;
use crate::level::block::block_state::BlockState;

pub trait BlockTrait {
    fn get_properties() -> BlockProperties;
    fn get_default_state() -> BlockState;
}