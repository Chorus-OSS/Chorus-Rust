use crate::block::state::block_state_value::BlockStateValue;
use crate::utils::utils;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockStateType {
    Boolean {
        name: String,
        default_value: bool,
        valid_values: Vec<bool>,
    },
    Int {
        name: String,
        min: i32,
        max: i32,
        default_value: i32,
        valid_values: Vec<i32>,
        bit_size: u8,
    },
    Enum {
        name: String,
        default_value: String,
        valid_values: Vec<String>,
        bit_size: u8,
    },
}

impl BlockStateType {
    pub fn create_boolean(name: &str, default_value: bool) -> BlockStateType {
        Self::Boolean {
            name: name.to_string(),
            default_value,
            valid_values: vec![true, false],
        }
    }

    pub fn create_int(name: &str, min: i32, max: i32, default_value: i32) -> Self {
        Self::Int {
            name: name.to_string(),
            min,
            max,
            default_value,
            valid_values: (min..=max).collect(),
            bit_size: utils::compute_required_bits(min, max),
        }
    }

    pub fn create_enum(name: &str, variants: &[&str], default_value: &str) -> Self {
        Self::Enum {
            name: name.to_string(),
            default_value: default_value.to_string(),
            valid_values: variants.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            bit_size: utils::compute_required_bits(0, (variants.len() - 1) as i32),
        }
    }

    pub fn get_bit_size(&self) -> &u8 {
        match self {
            BlockStateType::Boolean { .. } => &1,
            BlockStateType::Int { bit_size, .. } => bit_size,
            BlockStateType::Enum { bit_size, .. } => bit_size,
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            BlockStateType::Boolean { name, .. } => name,
            BlockStateType::Int { name, .. } => name,
            BlockStateType::Enum { name, .. } => name,
        }
    }

    pub fn create_default(&self) -> BlockStateValue {
        match self {
            BlockStateType::Boolean { default_value, .. } => {
                BlockStateValue::create_boolean(self.clone(), default_value.clone()).unwrap()
            }
            BlockStateType::Int { default_value, .. } => {
                BlockStateValue::create_int(self.clone(), default_value.clone()).unwrap()
            }
            BlockStateType::Enum { default_value, .. } => {
                BlockStateValue::create_enum(self.clone(), default_value.clone()).unwrap()
            }
        }
    }
}
