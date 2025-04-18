use crate::block::block_permutation::BlockPermutation;
use crate::block::block_state::BlockState;
use crate::level::level::Level;
use vek::Vec3;

pub struct Block  {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}

pub trait TBlock {
    fn get_state(&self) -> &BlockState;
    
    fn get_position(&self) -> &Vec3<i32>;
    fn get_layer(&self) -> &i32;
    
    fn get_level(&self) -> &Level;

    fn get_permutation() -> &'static BlockPermutation 
    where Self: Sized;
    fn get_default_state() -> &'static BlockState 
    where Self: Sized;
}