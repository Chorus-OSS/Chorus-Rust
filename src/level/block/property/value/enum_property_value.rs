use std::fmt::Debug;
use crate::level::block::property::r#type::block_property_type::{BlockPropertyType, BlockPropertyTypeTrait};
use crate::level::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::level::block::property::value::block_property_value::{BlockPropertyValue, BlockPropertyValueTrait};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumPropertyValue {
    property_type: EnumPropertyType,
    value: String,
}

impl EnumPropertyValue {
    pub fn new(property_type: EnumPropertyType, value: String) -> Self {
        Self {
            property_type,
            value
        }
    }
}

impl BlockPropertyValueTrait for EnumPropertyValue {
    type T = String;
    type S = String;
    
    fn get_property_type(&self) -> BlockPropertyType {
        BlockPropertyType::Enum(self.property_type.clone())
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn get_index(&self) -> i32 {
        self.property_type.get_valid_values().iter().position(|x| *x == self.value).unwrap() as i32
    }

    fn get_serialized_value(&self) -> String {
        format!("{:?}", self.value).to_lowercase()
    }
}