use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::block_property_value::BlockPropertyValue;
use crate::level::block::property::value::int_property_value::IntPropertyValue;
use crate::utils::utils;

#[derive(Clone, Debug, PartialEq)]
pub struct IntPropertyType {
    name: String,
    min: i32,
    max: i32,
    default_value: i32,
}

impl IntPropertyType {
    pub fn new(name: String, min: i32, max: i32, default_value: i32) -> Self {
        Self { name, min, max, default_value }
    }
    
    pub fn get_min(&self) -> i32 {
        self.min
    }
    
    pub fn get_max(&self) -> i32 {
        self.max
    }
}

impl BlockPropertyTypeTrait for IntPropertyType {
    type T = i32;
    
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_default_value(&self) -> i32 {
        self.default_value
    }

    fn get_valid_values(&self) -> Vec<i32> {
        (self.min..=self.max).collect()
    }

    fn get_bit_size(&self) -> u8 {
        utils::compute_required_bits(self.min, self.max)
    }

    fn create_value(&self, value: i32) -> BlockPropertyValue {
        BlockPropertyValue::Int(IntPropertyValue::new(self.clone(), value))
    }
}