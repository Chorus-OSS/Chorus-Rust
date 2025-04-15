use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::boolean_property_value::BooleanPropertyValue;
use crate::level::block::property::value::enum_property_value::EnumPropertyValue;
use crate::level::block::property::value::int_property_value::IntPropertyValue;

pub trait BlockPropertyValueTrait<T, P, S> {
    fn get_property_type(&self) -> P;
    fn get_value(&self) -> T;
    
    fn get_index(&self) -> i32;
    fn get_serialized_value(&self) -> S;
}

#[derive(Clone)]
pub enum BlockPropertyValue {
    Boolean(BooleanPropertyValue),
    Int(IntPropertyValue),
    Enum(EnumPropertyValue),
}