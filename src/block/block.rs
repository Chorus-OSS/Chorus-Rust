use vek::Vec3;
use crate::level::block::block_properties::BlockProperties;
use crate::level::block::block_state::BlockState;

pub trait Block {
    fn get_state(&self) -> &BlockState;
    
    fn get_position(&self) -> &Vec3<i32>;
    fn get_level(&self) -> &todo!("Level");
    
    fn get_layer(&self) -> i32 { 0 }
    fn get_friction_factor(&self) -> f32 { 0.6 }
    
    fn can_harvest_with_hand(&self) -> bool { true }
    
    fn get_tick_rate(&self) -> i32 { 10 }
    
    fn get_hardness(&self) -> f32 { 10.0 }
    fn get_resistance(&self) -> f32 { 1.0 }
    
    fn get_properties() -> BlockProperties;
    fn get_default_state() -> BlockState;
    
    fn is_solid(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
}