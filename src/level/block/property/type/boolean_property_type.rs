use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::block_property_value::BlockPropertyValue;
use crate::level::block::property::value::boolean_property_value::BooleanPropertyValue;

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanPropertyType {
    name: String,
    default_value: bool,
}

impl BooleanPropertyType {
    pub fn str_new(name: &str, default_value: bool) -> Self {
        Self {
            name: name.to_string(),
            default_value,
        }
    }
    
    pub fn new(name: String, default_value: bool) -> Self {
        Self { name, default_value }
    }
}

impl BlockPropertyTypeTrait for BooleanPropertyType {
    type T = bool;
    
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

    fn create_value(&self, value: bool) -> BlockPropertyValue {
        BlockPropertyValue::Boolean(BooleanPropertyValue::new(self.clone(), value))
    }
}