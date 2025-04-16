use crate::chorus::BLOCK_STATE_VERSION;
use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::block_property_value::{BlockPropertyValue, BlockPropertyValueTrait};
use crate::utils::hash_utils::HashUtils;
use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, HashMap};

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
                Self::compute_special_value(property_values.clone())
            }),
            state_tag: state_tag.unwrap_or_else(|| {
                Self::build_block_state_tag(identifier.clone(), property_values.clone())
            }),
        }
    }
    
    pub fn compute_special_value(property_values: Vec<BlockPropertyValue>) -> i16 {
        let mut special_value_bits: u8 = 0;
        for value in &property_values {
            special_value_bits += match value {
                BlockPropertyValue::Boolean(v) => v.get_property_type().get_bit_size(),
                BlockPropertyValue::Int(v) => v.get_property_type().get_bit_size(),
                BlockPropertyValue::Enum(v) => v.get_property_type().get_bit_size()
            }
        }
        
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
        let mut states: BTreeMap<String, nbtx::Value> = BTreeMap::new();
        for value in &property_values {
            match value { 
                BlockPropertyValue::Boolean(v) => {
                    states.insert(v.get_property_type().get_name(), nbtx::Value::Byte(if v.get_serialized_value() { 1 } else { 0 }));
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
        tag.insert(String::from("states"), nbtx::Value::TreeCompound(states));
        tag.insert(String::from("version"), nbtx::Value::Int(*BLOCK_STATE_VERSION));
        tag
    }
}