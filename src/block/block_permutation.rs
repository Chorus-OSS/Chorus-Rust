use crate::block::block_state::BlockState;
use crate::block::property::r#type::block_property_type::BlockProperty;
use crate::block::property::value::block_property_value::BlockPropertyValue;
use crate::utils::hash_utils::HashUtils;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::error::block_states_create::BlockStatesCreateError;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPermutation {
    identifier: String,
    properties: Vec<BlockProperty>,

    tags: HashSet<String>,
    friction_factor: f32,
    is_solid: bool,
    is_transparent: bool,
    hardness: f32,
    resistance: f32,
    burn_chance: i32,
    burn_ability: i32,

    special_value_map: HashMap<i16, BlockState>,
    special_value_bits: u8,
    
    default_state: BlockState,
}

impl BlockPermutation {
    pub fn create(
        identifier: &str,
        properties: Vec<BlockProperty>,
        tags: HashSet<String>,
        friction_factor: f32,
        is_solid: bool,
        is_transparent: bool,
        hardness: f32,
        resistance: f32,
        burn_chance: i32,
        burn_ability: i32,
    ) -> Result<Self, BlockStatesCreateError> {
        let identifier = identifier.to_string();
        
        let mut special_value_bits: u8 = 0;
        for val in &properties {
            special_value_bits += val.get_bit_size();
        }
        
        if special_value_bits > 16 {
            return Err(BlockStatesCreateError { 
                identifier: String::from(identifier), 
                properties: properties.clone(),
            });
        }
        
        if let Some((state_map, default_state)) = Self::init_states(identifier.clone(), properties.clone()) {
            Ok(Self {
                identifier,
                properties,
                
                tags,
                friction_factor,
                is_solid,
                is_transparent,
                hardness,
                resistance,
                burn_chance,
                burn_ability,
                
                special_value_map: state_map.iter().map(|(_, v)| {
                    (v.get_special_value(), v.clone())
                }).collect::<HashMap<i16, BlockState>>(),
                special_value_bits,
                
                default_state
            })
        } else {
            Err(BlockStatesCreateError {
                identifier: String::from(identifier),
                properties: properties.clone()
            })
        }
    }
    
    fn init_states(identifier: String, properties: Vec<BlockProperty>) -> Option<(HashMap<i32, BlockState>, BlockState)> {
        if properties.is_empty() {
            let block_state = BlockState::new(identifier.clone(), vec![], None, None, None);
            let mut special_value_map = HashMap::new();
            special_value_map.insert(block_state.get_hash(), block_state.clone());
            return Some((special_value_map, block_state));
        }
        
        let size = properties.len();

        let mut block_states: HashMap<i32, BlockState> = HashMap::new();
        let mut indices: Vec<usize> = vec![0; size];
        
        loop {
            let mut values: Vec<BlockPropertyValue> = vec![];
            for i in 0..size {
                let r#type = &properties[i];
                let val = match r#type {
                    BlockProperty::Boolean { valid_values, .. } => {
                        BlockPropertyValue::create_boolean(r#type.clone(), valid_values[indices[i]].clone()).unwrap()
                    }
                    BlockProperty::Int { valid_values, .. } => {
                        BlockPropertyValue::create_int(r#type.clone(), valid_values[indices[i]].clone()).unwrap()
                    }
                    BlockProperty::Enum { valid_values, .. } => {
                        BlockPropertyValue::create_enum(r#type.clone(), valid_values[indices[i]].clone()).unwrap()
                    }
                };
                values.push(val)
            }
            let state = BlockState::new(identifier.clone(), values, None, None, None);
            
            block_states.insert(state.get_hash(), state);
            
            let mut next = size - 1;
            while next >= 0 && (indices[next] + 1 >= match (&properties[next]) {
                BlockProperty::Boolean { valid_values, .. } => {
                    valid_values.len()
                }
                BlockProperty::Int { valid_values, .. } => {
                    valid_values.len()
                }
                BlockProperty::Enum { valid_values, .. } => {
                    valid_values.len()
                }
            }) {
                next -= 1;
            }
            
            if next < 0 { break; }
            
            indices[next] += 1;
            
            for i in next + 1 .. size {
                indices[i] = 0
            }
        }
        
        let default_state_hash = HashUtils::compute_block_state_hash(
            identifier.clone(),
            properties.iter().map(|v| {
                v.create_default()
            }).collect::<Vec<_>>(),
        );

        if let Some(state) = block_states.get(&default_state_hash) {
            Some((block_states.clone(), state.clone()))
        } else { None }
    }
    
    pub fn get_identifier(&self) -> String { 
        self.identifier.clone()
    }
    
    pub fn get_properties(&self) -> Vec<BlockProperty> {
        self.properties.clone()
    }
    
    pub fn get_special_value_map(&self) -> HashMap<i16, BlockState> {
        self.special_value_map.clone()
    }

    pub fn get_special_value_bits(&self) -> u8 {
        self.special_value_bits.clone()
    }

    pub fn get_default_state(&self) -> &BlockState {
        &self.default_state
    }
    
    pub fn get_block_state(&self, special_value: i16) -> Option<BlockState> {
        self.special_value_map.get(&special_value).cloned()
    }
    
    pub fn get_block_state_with_value(&self, value: BlockPropertyValue) -> Option<BlockState> {
        self.default_state.set_property_value(self.clone(), value)
    }
    
    pub fn get_block_state_with_values(&self, values: Vec<BlockPropertyValue>) -> Option<BlockState> {
        self.default_state.set_property_values(self.clone(), values)
    }
    
    pub fn has_block_state(&self, state: &BlockState) -> bool {
        self.special_value_map.contains_key(&state.get_special_value())
    }
    
    pub fn has_block_state_special_value(&self, special_value: i16) -> bool {
        self.special_value_map.contains_key(&special_value)
    }
    
    pub fn has_property(&self, property: BlockProperty) -> bool {
        self.properties.contains(&property)
    }
}
