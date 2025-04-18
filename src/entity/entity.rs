use vek::Vec3;
use crate::level::level::Level;

pub trait TEntity {
    fn get_position(&self) -> &Vec3<f32>;
    fn get_position_mut(&mut self) -> &mut Vec3<f32>;
    fn set_position(&mut self, pos: Vec3<f32>);
    
    fn get_velocity(&self) -> &Vec3<f32>;
    fn get_velocity_mut(&mut self) -> &mut Vec3<f32>;
    fn set_velocity(&mut self, vel: Vec3<f32>);
    
    fn get_level(&self) -> &Level;
    fn get_level_mut(&mut self) -> &mut Level;
    fn set_level(&mut self, level: Level);
}