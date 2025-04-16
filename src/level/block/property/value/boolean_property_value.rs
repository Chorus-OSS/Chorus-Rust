use crate::level::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::level::block::property::value::block_property_value::BlockPropertyValueTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanPropertyValue {
    property_type: BooleanPropertyType,
    value: bool,
}

impl BooleanPropertyValue {
    pub fn new(property_type: BooleanPropertyType, value: bool) -> Self {
        Self {
            property_type,
            value
        }
    }
}

impl BlockPropertyValueTrait for BooleanPropertyValue {
    type T = bool;
    type P = BooleanPropertyType;
    type S = i8;
    
    fn get_property_type(&self) -> BooleanPropertyType {
        self.property_type.clone()
    }

    fn get_value(&self) -> bool {
        self.value
    }

    fn get_index(&self) -> i32 {
        if (self.value) { 1 } else { 0 }
    }

    fn get_serialized_value(&self) -> i8 {
        if (self.value) { 1 } else { 0 }
    }
}