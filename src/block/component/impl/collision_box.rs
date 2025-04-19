use vek::Vec3;
use crate::block::component::block_component::BlockComponent;

pub struct CollisionBox {
    origin: Vec3<f32>,
    size: Vec3<f32>,
    enabled: bool
}

impl CollisionBox {
    pub fn default() -> Box<dyn BlockComponent> {
        Box::new(Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled: true
        })
    }

    pub fn create(origin: Vec3<f32>, size: Vec3<f32>) -> Box<dyn BlockComponent> {
        Box::new(Self {
            origin,
            size,
            enabled: true
        })
    }
    
    pub fn create_bool(enabled: bool) -> Box<dyn BlockComponent> {
        Box::new(Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled
        })
    }
}

impl BlockComponent for CollisionBox {
    fn get_identifier(&self) -> String {
        String::from("minecraft:collision_box")
    }
}