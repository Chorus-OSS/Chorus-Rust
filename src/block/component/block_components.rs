use crate::block::component::block_component::BlockComponent;
use crate::block::component::r#impl::collision_box::CollisionBox;
use std::collections::HashMap;

pub struct BlockComponents {
    components: HashMap<String, Box<dyn BlockComponent>>
}

impl BlockComponents {
    pub fn create(components: Vec<Box<dyn BlockComponent>>) -> Self {
        let mut map: HashMap<String, Box<dyn BlockComponent>> = HashMap::new();
        
        let mut defaults = vec![
            CollisionBox::default(),
            
        ];
        defaults.extend(components);
        
        let mut components = defaults;
        
        for component in components {
            map.insert(component.get_identifier(), component);
        }
        
        Self {
            components: map
        }
    }
}