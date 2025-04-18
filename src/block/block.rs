use vek::Vec3;
use crate::block::transparent::block_transparent::TBlockTransparent;
use crate::level::block::block_properties::TBlockProperties;
use crate::level::block::block_states::BlockStates;
use crate::level::block::block_state::BlockState;
use crate::level::level::Level;

pub trait TBlock {
    fn get_state(&self) -> &BlockState;
    
    fn get_position(&self) -> &Vec3<i32>;
    fn get_layer(&self) -> &i32;
    
    fn get_level(&self) -> &Level;
    
    fn get_tick_rate(&self) -> i32 { 10 }


    fn get_states() -> &'static BlockStates where Self: Sized;
    fn get_default_state() -> &'static BlockState where Self: Sized;
    fn get_properties() -> &'static dyn TBlockProperties where Self: Sized;
    
    fn as_transparent_block(&self) -> Option<&dyn TBlockTransparent> {
        None
    }
}