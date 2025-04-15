use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
use crate::level::block::property::value::int_property_value::IntPropertyValue;
use crate::utils::utils;

#[derive(Clone)]
pub struct IntPropertyType {
    name: String,
    min: i32,
    max: i32,
    default_value: i32,
}

impl IntPropertyType {
    pub fn get_min(&self) -> i32 {
        self.min
    }
    
    pub fn get_max(&self) -> i32 {
        self.max
    }
}

impl BlockPropertyTypeTrait<i32, IntPropertyValue> for IntPropertyType {
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

    fn create_value(&self, value: i32) -> IntPropertyValue {
        IntPropertyValue::new(self.clone(), value)
    }
}