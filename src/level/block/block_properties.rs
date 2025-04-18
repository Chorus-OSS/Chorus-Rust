use std::collections::HashSet;

pub trait TBlockProperties {
    fn get_identifier(&self) -> &String;
    fn get_tags(&self) -> &HashSet<String>;

    fn get_friction_factor(&self) -> f32 { 0.6 }

    fn can_harvest_with_hand(&self) -> bool { true }

    fn is_solid(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
    
    fn get_hardness(&self) -> f32 { 10.0 }
    fn get_resistance(&self) -> f32 { 1.0 }
    
    fn get_burn_chance(&self) -> i32 { 0 }
    fn get_burn_ability(&self) -> i32 { 0 }
}

#[derive(Clone, Debug)]
pub struct BlockProperties {
    identifier: String,
    tags: HashSet<String>,
}

impl BlockProperties {
    pub fn new(identifier: &str, tags: Option<HashSet<String>>) -> Self {
        Self {
            identifier: identifier.to_string(),
            tags: tags.unwrap_or_else(HashSet::new),
        }
    }
}

impl TBlockProperties for BlockProperties {
    fn get_identifier(&self) -> &String { 
        &self.identifier
    }
    
    fn get_tags(&self) -> &HashSet<String> { 
        &self.tags
    }
}
