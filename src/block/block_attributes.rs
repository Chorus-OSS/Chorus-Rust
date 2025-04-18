use std::collections::HashSet;

pub trait TBlockAttributes {}

#[derive(Clone, Debug)]
pub struct BlockAttributes {
    pub identifier: String,
    pub tags: HashSet<String>,
    pub friction_factor: f32,
    pub unbreakable: bool,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub hardness: f32,
    pub resistance: f32,
    pub burn_chance: i32,
    pub burn_ability: i32,
}

impl BlockAttributes {
    pub fn default(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            tags: HashSet::new(),
            friction_factor: 0.6,
            unbreakable: false,
            is_solid: true,
            is_transparent: false,
            hardness: 10.0,
            resistance: 1.0,
            burn_chance: 0,
            burn_ability: 0,
        }
    }
}
