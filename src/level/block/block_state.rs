use crate::chorus::BLOCK_STATE_VERSION;
use crate::level::block::property::r#type::block_property_type::{BlockPropertyType, BlockPropertyTypeTrait};
use crate::level::block::property::value::block_property_value::{BlockPropertyValue, BlockPropertyValueTrait};
use crate::utils::hash_utils::HashUtils;
use std::collections::HashMap;
use crate::level::block::block_states::BlockStates;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockState {
    identifier: String,
    hash: i32,
    special_value: i16,
    property_values: Vec<BlockPropertyValue>,
    state_tag: HashMap<String, nbtx::Value>
}

impl BlockState {
    pub fn new(identifier: String, property_values: Vec<BlockPropertyValue>, hash: Option<i32>, special_value: Option<i16>, state_tag: Option<HashMap<String, nbtx::Value>>) -> Self {
        Self {
            identifier: identifier.clone(),
            property_values: property_values.clone(),
            hash: hash.unwrap_or_else(|| {
                HashUtils::compute_block_state_hash(identifier.clone(), property_values.clone())
            }),
            special_value: special_value.unwrap_or_else(|| {
                Self::compute_special_value(property_values.clone(), None)
            }),
            state_tag: state_tag.unwrap_or_else(|| {
                Self::build_block_state_tag(identifier.clone(), property_values.clone())
            }),
        }
    }
    
    pub fn get_hash(&self) -> i32 {
        self.hash
    }
    
    pub fn get_special_value(&self) -> i16 {
        self.special_value
    }
    
    pub fn get_property_value(&self, property: BlockPropertyType) -> Option<BlockPropertyValue> {
        for val in &self.property_values {
            if match val {
                BlockPropertyValue::Boolean(v) => v.get_property_type(),
                BlockPropertyValue::Int(v) => v.get_property_type(),
                BlockPropertyValue::Enum(v) => v.get_property_type(),
            } == property {
                return Some(val.clone());
            }
        }
        None
    }
    
    pub fn set_property_value(&self, properties: BlockStates, value: BlockPropertyValue) -> Option<BlockState> {
        let mut success = false;
        let mut new_property_values: Vec<BlockPropertyValue> = Vec::new();
        for v in &self.property_values {
            if (*v == value) {
                success = true;
                new_property_values.push(value.clone())
            } else { new_property_values.push(v.clone()) }
        }
        
        match success {
            true => self.get_new_block_state(properties, new_property_values),
            false => None
        }
    }
    
    pub fn set_property_values(&self, properties: BlockStates, values: Vec<BlockPropertyValue>) -> Option<BlockState> {
        let mut success_count: usize = 0;
        
        let mut new_property_values: Vec<BlockPropertyValue> = Vec::new();
        'f: for v in &self.property_values {
            for j in &values {
                if (*v == *j) {
                    success_count += 1;
                    new_property_values.push(j.clone());
                    continue 'f
                }
            }
            new_property_values.push(v.clone());
        }
        
        match success_count == values.len() {
            true => self.get_new_block_state(properties, new_property_values),
            false => None
        }
    }
    
    pub fn compute_special_value(property_values: Vec<BlockPropertyValue>, special_value_bits: Option<u8>) -> i16 {
        let mut special_value_bits = special_value_bits.unwrap_or_else(|| {
            let mut bits: u8 = 0;
            for value in &property_values {
                bits += match value {
                    BlockPropertyValue::Boolean(v) => v.get_property_type().get_bit_size(),
                    BlockPropertyValue::Int(v) => v.get_property_type().get_bit_size(),
                    BlockPropertyValue::Enum(v) => v.get_property_type().get_bit_size()
                }
            }
            bits
        });
        
        let mut special_value: i16 = 0;
        for value in &property_values {
            let (bit_size, index) = match value { 
                BlockPropertyValue::Boolean(v) => (v.get_property_type().get_bit_size(), v.get_index()),
                BlockPropertyValue::Int(v) => (v.get_property_type().get_bit_size(), v.get_index()),
                BlockPropertyValue::Enum(v) => (v.get_property_type().get_bit_size(), v.get_index())
            };
            special_value = (special_value as i32 | (index << (special_value_bits - bit_size))) as i16;
            special_value_bits = special_value_bits - bit_size;
        }
        special_value
    }
    
    fn build_block_state_tag(identifier: String, property_values: Vec<BlockPropertyValue>) -> HashMap<String, nbtx::Value> {
        let mut states: HashMap<String, nbtx::Value> = HashMap::new();
        for value in &property_values {
            match value { 
                BlockPropertyValue::Boolean(v) => {
                    states.insert(v.get_property_type().get_name(), nbtx::Value::Byte(v.get_serialized_value()));
                }
                BlockPropertyValue::Int(v) => {
                    states.insert(v.get_property_type().get_name(), nbtx::Value::Int(v.get_serialized_value()));
                }
                BlockPropertyValue::Enum(v) => {
                    states.insert(v.get_property_type().get_name(), nbtx::Value::String(v.get_serialized_value()));
                }
            }
        }
        let mut tag: HashMap<String, nbtx::Value> = HashMap::new();
        tag.insert(String::from("name"), nbtx::Value::String(identifier));
        tag.insert(String::from("states"), nbtx::Value::Compound(states));
        tag.insert(String::from("version"), nbtx::Value::Int(*BLOCK_STATE_VERSION));
        tag
    }
    
    fn get_new_block_state(&self, properties: BlockStates, values: Vec<BlockPropertyValue>) -> Option<BlockState> {
        let bits: u8 = properties.get_special_value_bits();
        match (bits <= 16) {
            true => properties.get_block_state(Self::compute_special_value(values, Some(bits))),
            false => None
        }
    }
}