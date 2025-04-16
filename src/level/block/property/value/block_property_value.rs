use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::boolean_property_value::BooleanPropertyValue;
use crate::level::block::property::value::enum_property_value::EnumPropertyValue;
use crate::level::block::property::value::int_property_value::IntPropertyValue;

pub trait BlockPropertyValueTrait {
    type T;
    type P : BlockPropertyTypeTrait<T = Self::T>;
    type S;
    
    fn get_property_type(&self) -> Self::P;
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