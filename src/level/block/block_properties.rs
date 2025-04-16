use crate::level::block::block_state::BlockState;
use crate::level::block::property::r#type::block_property_type::{BlockPropertyType, BlockPropertyTypeTrait};
use crate::level::block::property::value::block_property_value::BlockPropertyValue;
use crate::utils::hash_utils::HashUtils;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockProperties {
    identifier: String,
    tags: HashSet<String>,
    properties: Vec<BlockPropertyType>,
    
    default_state: BlockState,
    special_value_bits: u8,
    special_value_map: HashMap<i16, BlockState>,
}

impl BlockProperties {
    pub fn create(identifier: String, properties: Vec<BlockPropertyType>, tags: Option<HashSet<String>>) -> Option<Self> {
        // TODO: register(identifier, tags);
        
        let mut special_value_bits: u8 = 0;
        for val in &properties {
            special_value_bits += val.get_bit_size();
        }
        
        if special_value_bits > 16 {
            return None;
        }
        
        if let Some((state_map, default_state)) = Self::init_states(identifier.clone(), properties.clone()) {
            Some(Self {
                identifier,
                properties,
                tags: tags.unwrap_or_else(|| HashSet::new()),

                default_state,
                special_value_bits,
                special_value_map: state_map.iter().map(|(_, v)| {
                    (v.get_special_value(), v)
                }).collect()
            })
        } else { None }
    }
    
    fn init_states(identifier: String, properties: Vec<BlockPropertyType>) -> Option<(HashMap<i32, BlockState>, BlockState)> {
        if properties.is_empty() {
            let block_state = BlockState::new(identifier.clone(), vec![], None, None, None);
            let mut special_value_map = HashMap::new();
            special_value_map.insert(block_state.get_hash(), block_state.clone());
            return Some((special_value_map, block_state));
        }
        
        let size = properties.len();

        let mut block_states: HashMap<i32, BlockState> = HashMap::new();
        let mut indices: Vec<i32> = vec![0; size];
        
        loop {
            let mut values: Vec<BlockPropertyValue> = vec![];
            for i in 0..size {
                let r#type = &properties[i];
                let val = match r#type {
                    BlockPropertyType::Boolean(v) => {
                        v.create_value(v.get_valid_values()[indices[i]].clone())
                    }
                    BlockPropertyType::Int(v) => {
                        v.create_value(v.get_valid_values()[indices[i]].clone())
                    }
                    BlockPropertyType::Enum(v) => {
                        v.create_value(v.get_valid_values()[indices[i]].clone())
                    }
                };
                values.push(val)
            }
            let state = BlockState::new(identifier.clone(), values, None, None, None);
            
            block_states.insert(state.get_hash(), state);
            
            let mut next = size - 1;
            while next >= 0 && (indices[next] + 1 >= match (&properties[next]) {
                BlockPropertyType::Boolean(v) => {
                    v.get_valid_values().len()
                }
                BlockPropertyType::Int(v) => {
                    v.get_valid_values().len()
                }
                BlockPropertyType::Enum(v) => {
                    v.get_valid_values().len()
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
                match v {
                    BlockPropertyType::Boolean(v) => {
                        v.create_value(v.get_default_value())
                    }
                    BlockPropertyType::Int(v) => {
                        v.create_value(v.get_default_value())
                    }
                    BlockPropertyType::Enum(v) => {
                        v.create_value(v.get_default_value())
                    }
                }
            }).collect::<Vec<_>>(),
        );

        if let Some(state) = block_states.get(&default_state_hash) {
            Some((block_states, state))
        } else { None }
    }
    
    pub fn get_identifier(&self) -> String { 
        self.identifier.clone()
    }
    
    pub fn get_tags(&self) -> HashSet<String> {
        self.tags.clone()
    }
    
    pub fn get_properties(&self) -> Vec<BlockPropertyType> {
        self.properties.clone()
    }
    
    pub fn get_default_state(&self) -> BlockState {
        self.default_state.clone()
    }
    
    pub fn get_special_value_bits(&self) -> u8 {
        self.special_value_bits.clone()
    }
    
    pub fn get_special_value_map(&self) -> HashMap<i16, BlockState> {
        self.special_value_map.clone()
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
    
    pub fn has_property(&self, property: BlockPropertyType) -> bool {
        self.properties.contains(&property)
    }
}
