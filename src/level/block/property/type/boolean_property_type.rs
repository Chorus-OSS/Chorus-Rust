use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::boolean_property_value::BooleanPropertyValue;

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanPropertyType {
    name: String,
    default_value: bool,
}

impl BlockPropertyTypeTrait for BooleanPropertyType {
    type T = bool;
    type V = BooleanPropertyValue;
    
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_default_value(&self) -> bool {
        self.default_value
    }

    fn get_valid_values(&self) -> Vec<bool> {
        vec![true, false]
    }

    fn get_bit_size(&self) -> u8 {
        1
    }

    fn create_value(&self, value: bool) -> BooleanPropertyValue {
        BooleanPropertyValue::new(self.clone(), value)
    }
}