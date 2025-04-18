use crate::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::block::property::value::enum_property_value::EnumPropertyValue;
use crate::utils::utils;
use std::fmt::{Debug, Display};
use crate::block::property::value::block_property_value::BlockPropertyValue;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumPropertyType {
    name: String,
    variants: Vec<String>,
    default_value: String,
}

impl EnumPropertyType {
    pub fn str_new(name: &str, variants: &[&str], default_value: &str) -> Self {
        Self { 
            name: name.to_string(),
            variants: variants.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            default_value: default_value.to_string(),
        }
    }
    
    pub fn new(name: String, variants: Vec<String>, default_value: String) -> Self {
        Self { name, variants, default_value }
    }
}

impl BlockPropertyTypeTrait for EnumPropertyType {
    type T = String;
    
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_default_value(&self) -> String {
        self.default_value.clone()
    }

    fn get_valid_values(&self) -> Vec<String> {
        self.variants.clone()
    }

    fn get_bit_size(&self) -> u8 {
        utils::compute_required_bits(0, (self.variants.len() - 1) as i32)
    }

    fn create_value(&self, value: String) -> BlockPropertyValue {
        BlockPropertyValue::Enum(EnumPropertyValue::new(self.clone(), value))
    }
}