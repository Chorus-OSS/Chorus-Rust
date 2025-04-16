use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::enum_property_value::EnumPropertyValue;
use crate::utils::utils;
use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct EnumPropertyType {
    name: String,
    variants: Vec<String>,
    default_value: String,
}

impl BlockPropertyTypeTrait for EnumPropertyType {
    type T = String;
    type V = EnumPropertyValue;
    
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

    fn create_value(&self, value: String) -> EnumPropertyValue {
        EnumPropertyValue::new(self.clone(), value)
    }
}