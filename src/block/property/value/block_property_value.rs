use crate::block::property::r#type::block_property_type::{BlockPropertyType, BlockPropertyTypeTrait};
use crate::block::property::value::boolean_property_value::BooleanPropertyValue;
use crate::block::property::value::enum_property_value::EnumPropertyValue;
use crate::block::property::value::int_property_value::IntPropertyValue;

pub trait BlockPropertyValueTrait {
    type T;
    type S;
    
    fn get_property_type(&self) -> BlockPropertyType;
    fn get_value(&self) -> Self::T;
    
    fn get_index(&self) -> i32;
    fn get_serialized_value(&self) -> Self::S;
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockPropertyValue {
    Boolean(BooleanPropertyValue),
    Int(IntPropertyValue),
    Enum(EnumPropertyValue),
}

impl BlockPropertyValue {
    pub fn get_index(&self) -> i32 {
        match self {
            BlockPropertyValue::Boolean(v) => v.get_index(),
            BlockPropertyValue::Int(v) => v.get_index(),
            BlockPropertyValue::Enum(v) => v.get_index(),
        }
    }
    
    pub fn get_property_type(&self) -> BlockPropertyType {
        match self {
            BlockPropertyValue::Boolean(v) => v.get_property_type(),
            BlockPropertyValue::Int(v) => v.get_property_type(),
            BlockPropertyValue::Enum(v) => v.get_property_type(),
        }
    }
}