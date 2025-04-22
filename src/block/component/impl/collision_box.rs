use crate::block::component::block_component::BlockComponent;
use crate::block::component::block_components::BlockComponents;
use vek::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct CollisionBox {
    origin: Vec3<f32>,
    size: Vec3<f32>,
    enabled: bool,
}

impl CollisionBox {
    pub fn default() -> BlockComponent {
        BlockComponent::CollisionBox(Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled: true,
        })
    }

    pub fn create(origin: Vec3<f32>, size: Vec3<f32>) -> BlockComponent {
        BlockComponent::CollisionBox(Self {
            origin,
            size,
            enabled: true,
        })
    }

    pub fn create_bool(enabled: bool) -> BlockComponent {
        BlockComponent::CollisionBox(Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled,
        })
    }
}
