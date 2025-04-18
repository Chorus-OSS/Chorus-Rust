use crate::block::property::block_property_value::BlockPropertyValue;
use crate::utils::utils;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockProperty {
    Boolean {
        name: String,
        default_value: bool,
        valid_values: Vec<bool>,
    },
    Int {
        name: String,
        min: i32,
        max: i32,
        default_value: i32,
        valid_values: Vec<i32>,
        bit_size: u8,
    },
    Enum {
        name: String,
        default_value: String,
        valid_values: Vec<String>,
        bit_size: u8,
    },
}

impl BlockProperty {
    pub fn create_boolean(name: &str, default_value: bool) -> BlockProperty {
        Self::Boolean {
            name: name.to_string(),
            default_value,
            valid_values: vec![true, false],
        }
    }

    pub fn create_int(name: &str, min: i32, max: i32, default_value: i32) -> Self {
        Self::Int {
            name: name.to_string(),
            min,
            max,
            default_value,
            valid_values: (min..=max).collect(),
            bit_size: utils::compute_required_bits(min, max),
        }
    }
    
    pub fn create_enum(name: &str, variants: &[&str], default_value: &str) -> Self {
        Self::Enum {
            name: name.to_string(),
            default_value: default_value.to_string(),
            valid_values: variants.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            bit_size: utils::compute_required_bits(0, (variants.len() - 1) as i32),
        }
    }
    
    pub fn get_bit_size(&self) -> &u8 {
        match self {
            BlockProperty::Boolean { .. } => &1,
            BlockProperty::Int { bit_size, .. } => bit_size,
            BlockProperty::Enum { bit_size, .. } => bit_size,
        }
    }
    
    pub fn get_name(&self) -> &String {
        match self {
            BlockProperty::Boolean { name, .. } => name,
            BlockProperty::Int { name, .. } => name,
            BlockProperty::Enum { name, .. } => name,
        }
    }
    
    pub fn create_default(&self) -> BlockPropertyValue {
        match self {
            BlockProperty::Boolean { default_value, .. } => BlockPropertyValue::create_boolean(self.clone(), default_value.clone()).unwrap(),
            BlockProperty::Int { default_value, .. } => BlockPropertyValue::create_int(self.clone(), default_value.clone()).unwrap(),
            BlockProperty::Enum { default_value, .. } => BlockPropertyValue::create_enum(self.clone(), default_value.clone()).unwrap(),
        }
    }
}