use std::any::Any;
use crate::level::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::level::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::level::block::property::r#type::int_property_type::IntPropertyType;
use crate::level::block::property::value::block_property_value::BlockPropertyValueTrait;

pub trait BlockPropertyTypeTrait {
    type T;
    type V : BlockPropertyValueTrait<T = Self::T>;
    fn get_name(&self) -> String;
    fn get_default_value(&self) -> Self::T;
    fn get_valid_values(&self) -> Vec<Self::T>;
    
    fn get_bit_size(&self) -> u8;
    
    fn create_value(&self, value: Self::T) -> Self::V;
}

pub enum BlockPropertyType {
    Boolean(BooleanPropertyType),
    Int(IntPropertyType),
    Enum(EnumPropertyType),
}