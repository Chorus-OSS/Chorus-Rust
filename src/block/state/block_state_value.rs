use crate::block::state::block_state_type::BlockStateType;
use crate::error::mismatched_block_state_type::MismatchedBlockStateTypeError;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockStateValue {
    Boolean {
        property_type: BlockStateType,
        value: bool,
        index: i32,
        serialized_value: u8,
    },
    Int {
        property_type: BlockStateType,
        value: i32,
        index: i32,
        serialized_value: i32,
    },
    Enum {
        property_type: BlockStateType,
        value: String,
        index: i32,
        serialized_value: String,
    },
}

impl BlockStateValue {
    pub fn create_boolean(
        property_type: BlockStateType,
        value: bool,
    ) -> Result<Self, MismatchedBlockStateTypeError> {
        match &property_type {
            BlockStateType::Boolean { .. } => Ok(Self::Boolean {
                property_type: property_type.clone(),
                value,
                index: if (value) { 1 } else { 0 },
                serialized_value: if (value) { 1 } else { 0 },
            }),
            _ => Err(MismatchedBlockStateTypeError {
                found: property_type,
                expected: String::from("BlockProperty::Boolean"),
            }),
        }
    }

    pub fn create_int(
        property_type: BlockStateType,
        value: i32,
    ) -> Result<Self, MismatchedBlockStateTypeError> {
        match &property_type {
            BlockStateType::Int { min, .. } => Ok(Self::Int {
                property_type: property_type.clone(),
                value,
                index: value - min,
                serialized_value: value,
            }),
            _ => Err(MismatchedBlockStateTypeError {
                found: property_type,
                expected: String::from("BlockProperty::Int"),
            }),
        }
    }

    pub fn create_enum(
        property_type: BlockStateType,
        value: String,
    ) -> Result<Self, MismatchedBlockStateTypeError> {
        match &property_type {
            BlockStateType::Enum { valid_values, .. } => Ok(Self::Enum {
                property_type: property_type.clone(),
                value: value.clone(),
                index: valid_values.iter().position(|x| x == &value).unwrap() as i32,
                serialized_value: value.clone(),
            }),
            _ => Err(MismatchedBlockStateTypeError {
                found: property_type,
                expected: String::from("BlockProperty::Enum"),
            }),
        }
    }

    pub fn get_index(&self) -> &i32 {
        match self {
            BlockStateValue::Boolean { index, .. } => index,
            BlockStateValue::Int { index, .. } => index,
            BlockStateValue::Enum { index, .. } => index,
        }
    }

    pub fn get_property_type(&self) -> &BlockStateType {
        match self {
            BlockStateValue::Boolean { property_type, .. } => property_type,
            BlockStateValue::Int { property_type, .. } => property_type,
            BlockStateValue::Enum { property_type, .. } => property_type,
        }
    }
}
