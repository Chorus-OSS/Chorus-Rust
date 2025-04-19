use vek::Vec3;
use crate::level::level::Level;

pub struct Entity {
    position: Vec3<f32>,
    velocity: Vec3<f32>,
    level: Level,
}