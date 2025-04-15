use crate::level::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::level::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::level::block::property::r#type::int_property_type::IntPropertyType;

pub trait BlockPropertyTypeTrait<T, V> {
    fn get_name(&self) -> String;
    fn get_default_value(&self) -> T;
    fn get_valid_values(&self) -> Vec<T>;
    
    fn get_bit_size(&self) -> u8;
    
    fn create_value(&self, value: T) -> V;
}

pub enum BlockPropertyType {
    Boolean(BooleanPropertyType),
    Int(IntPropertyType),
    Enum(EnumPropertyType),
}