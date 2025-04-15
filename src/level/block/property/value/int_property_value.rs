use crate::level::block::property::r#type::int_property_type::IntPropertyType;
use crate::level::block::property::value::block_property_value::BlockPropertyValueTrait;

#[derive(Clone)]
pub struct IntPropertyValue {
    property_type: IntPropertyType,
    value: i32,
}

impl IntPropertyValue {
    pub fn new(property_type: IntPropertyType, value: i32) -> Self {
        Self { 
            property_type, 
            value 
        }
    }
}

impl BlockPropertyValueTrait<i32, IntPropertyType, i32> for IntPropertyValue {
    fn get_property_type(&self) -> IntPropertyType {
        self.property_type.clone()
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_index(&self) -> i32 {
        self.value - self.property_type.get_min()
    }

    fn get_serialized_value(&self) -> i32 {
        self.value
    }
}