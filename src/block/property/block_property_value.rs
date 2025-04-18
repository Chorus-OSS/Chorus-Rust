use crate::block::property::block_property_type::BlockProperty;
use crate::error::mismatched_block_property::MismatchedBlockPropertyError;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockPropertyValue {
    Boolean {
        property_type: BlockProperty,
        value: bool,
        index: i32,
        serialized_value: u8,
    },
    Int {
        property_type: BlockProperty,
        value: i32,
        index: i32,
        serialized_value: i32,
    },
    Enum {
        property_type: BlockProperty,
        value: String,
        index: i32,
        serialized_value: String,
    },
}

impl BlockPropertyValue {
    pub fn create_boolean(property_type: BlockProperty, value: bool) -> Result<Self, MismatchedBlockPropertyError> {
        match &property_type {
            BlockProperty::Boolean { .. } => Ok(
                Self::Boolean {
                    property_type: property_type.clone(),
                    value,
                    index: if (value) { 1 } else { 0 },
                    serialized_value: if (value) { 1 } else { 0 },
                }
            ),
            _ => Err(
                MismatchedBlockPropertyError {
                    found: property_type,
                    expected: String::from("BlockProperty::Boolean"),
                }
            )
        }
    }
    
    pub fn create_int(property_type: BlockProperty, value: i32) -> Result<Self, MismatchedBlockPropertyError> {
        match &property_type {
            BlockProperty::Int { min, .. } => Ok(
                Self::Int {
                    property_type: property_type.clone(),
                    value,
                    index: value - min,
                    serialized_value: value,
                }
            ),
            _ => Err(
                MismatchedBlockPropertyError {
                    found: property_type,
                    expected: String::from("BlockProperty::Int"),
                }
            )
        }
    }
    
    pub fn create_enum(property_type: BlockProperty, value: String) -> Result<Self, MismatchedBlockPropertyError> {
        match &property_type {
            BlockProperty::Enum { valid_values, .. } => Ok(
                Self::Enum {
                    property_type: property_type.clone(),
                    value: value.clone(),
                    index: valid_values.iter().position(|x| x == &value).unwrap() as i32,
                    serialized_value: value.clone(),
                }
            ),
            _ => Err(
                MismatchedBlockPropertyError {
                    found: property_type,
                    expected: String::from("BlockProperty::Enum"),
                }
            )
        }
    }
    
    pub fn get_index(&self) -> &i32 {
        match self {
            BlockPropertyValue::Boolean { index, .. } => index,
            BlockPropertyValue::Int { index, .. } => index,
            BlockPropertyValue::Enum { index, .. } => index,
        }
    }
    
    pub fn get_property_type(&self) -> &BlockProperty {
        match self {
            BlockPropertyValue::Boolean { property_type, .. } => property_type,
            BlockPropertyValue::Int { property_type, .. } => property_type,
            BlockPropertyValue::Enum { property_type, .. } => property_type,
        }
    }
}