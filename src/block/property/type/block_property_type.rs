use std::any::Any;
use crate::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::block::property::r#type::int_property_type::IntPropertyType;
use crate::block::property::value::block_property_value::{BlockPropertyValue, BlockPropertyValueTrait};

pub trait BlockPropertyTypeTrait {
    type T;
    fn get_name(&self) -> String;
    fn get_default_value(&self) -> Self::T;
    fn get_valid_values(&self) -> Vec<Self::T>;
    
    fn get_bit_size(&self) -> u8;
    
    fn create_value(&self, value: Self::T) -> BlockPropertyValue;
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockPropertyType {
    Boolean(BooleanPropertyType),
    Int(IntPropertyType),
    Enum(EnumPropertyType),
}

impl BlockPropertyType {
    pub fn get_bit_size(&self) -> u8 {
        match self {
            BlockPropertyType::Boolean(v) => v.get_bit_size(),
            BlockPropertyType::Int(v) => v.get_bit_size(),
            BlockPropertyType::Enum(v) => v.get_bit_size()
        }
    }
    
    pub fn get_name(&self) -> String {
        match self {
            BlockPropertyType::Boolean(v) => v.get_name(),
            BlockPropertyType::Int(v) => v.get_name(),
            BlockPropertyType::Enum(v) => v.get_name(),
        }
    }
}