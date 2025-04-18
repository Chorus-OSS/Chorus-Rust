use std::collections::HashSet;

pub trait TBlockAttributes {}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockAttributes {
    pub tags: HashSet<String>,
    pub friction_factor: f32,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub hardness: f32,
    pub resistance: f32,
    pub burn_chance: i32,
    pub burn_ability: i32,
}
