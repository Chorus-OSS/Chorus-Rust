use crate::block::block_attributes::BlockAttributes;
use crate::block::block_state::BlockState;
use crate::block::block_states::BlockStates;
use crate::level::level::Level;
use vek::Vec3;

pub trait TBlock {
    fn get_state(&self) -> &BlockState;
    
    fn get_position(&self) -> &Vec3<i32>;
    fn get_layer(&self) -> &i32;
    
    fn get_level(&self) -> &Level;

    fn get_states() -> &'static BlockStates 
    where Self: Sized;
    fn get_default_state() -> &'static BlockState 
    where Self: Sized;
    fn get_attributes() -> &'static BlockAttributes
    where Self: Sized;
}