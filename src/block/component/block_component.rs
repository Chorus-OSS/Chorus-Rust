use std::fmt::Debug;
use crate::block::component::r#impl::collision_box::CollisionBox;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockComponent {
    CollisionBox(CollisionBox),
}

impl BlockComponent {
    pub fn get_identifier(&self) -> String {
        match self {
            BlockComponent::CollisionBox(..) => String::from("minecraft:collision_box")
        }
    }
}