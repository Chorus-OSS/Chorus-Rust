use crate::block::block_type::BlockType;
use crate::block::state::block_state_type::BlockStateType;
use crate::block::state::block_state_value::BlockStateValue;
use crate::chorus::BLOCK_STATE_VERSION;
use crate::utils::hash_utils::HashUtils;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPermutation {
    identifier: String,
    hash: i32,
    special_value: i16,
    state_values: Vec<BlockStateValue>,
    state_tag: HashMap<String, nbtx::Value>,
}

impl BlockPermutation {
    pub fn new(
        identifier: String,
        property_values: Vec<BlockStateValue>,
        hash: Option<i32>,
        special_value: Option<i16>,
        state_tag: Option<HashMap<String, nbtx::Value>>,
    ) -> Self {
        Self {
            identifier: identifier.clone(),
            state_values: property_values.clone(),
            hash: hash.unwrap_or_else(|| {
                HashUtils::compute_block_permutation_hash(
                    identifier.clone(),
                    property_values.clone(),
                )
            }),
            special_value: special_value
                .unwrap_or_else(|| Self::compute_special_value(property_values.clone(), None)),
            state_tag: state_tag.unwrap_or_else(|| {
                Self::build_block_state_tag(identifier.clone(), property_values.clone())
            }),
        }
    }

    pub fn get_hash(&self) -> i32 {
        self.hash
    }

    pub fn get_special_value(&self) -> i16 {
        self.special_value
    }

    pub fn get_property_value(&self, property: BlockStateType) -> Option<BlockStateValue> {
        for val in &self.state_values {
            if val.get_property_type() == &property {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn set_property_value(
        &self,
        properties: BlockType,
        value: BlockStateValue,
    ) -> Option<BlockPermutation> {
        let mut success = false;
        let mut new_property_values: Vec<BlockStateValue> = Vec::new();
        for v in &self.state_values {
            if (*v == value) {
                success = true;
                new_property_values.push(value.clone())
            } else {
                new_property_values.push(v.clone())
            }
        }

        match success {
            true => self.get_new_block_state(properties, new_property_values),
            false => None,
        }
    }

    pub fn set_property_values(
        &self,
        properties: BlockType,
        values: Vec<BlockStateValue>,
    ) -> Option<BlockPermutation> {
        let mut success_count: usize = 0;

        let mut new_property_values: Vec<BlockStateValue> = Vec::new();
        'f: for v in &self.state_values {
            for j in &values {
                if (*v == *j) {
                    success_count += 1;
                    new_property_values.push(j.clone());
                    continue 'f;
                }
            }
            new_property_values.push(v.clone());
        }

        match success_count == values.len() {
            true => self.get_new_block_state(properties, new_property_values),
            false => None,
        }
    }

    pub fn compute_special_value(
        property_values: Vec<BlockStateValue>,
        special_value_bits: Option<u8>,
    ) -> i16 {
        let mut special_value_bits = special_value_bits.unwrap_or_else(|| {
            let mut bits: u8 = 0;
            for value in &property_values {
                bits += value.get_property_type().get_bit_size();
            }
            bits
        });

        let mut special_value: i16 = 0;
        for value in &property_values {
            let bit_size = value.get_property_type().get_bit_size();
            let index = value.get_index();

            special_value =
                (special_value as i32 | (index << (special_value_bits - bit_size))) as i16;
            special_value_bits = special_value_bits - bit_size;
        }
        special_value
    }

    fn build_block_state_tag(
        identifier: String,
        property_values: Vec<BlockStateValue>,
    ) -> HashMap<String, nbtx::Value> {
        let mut states: HashMap<String, nbtx::Value> = HashMap::new();
        for value in &property_values {
            match value {
                BlockStateValue::Boolean {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::Byte(serialized_value.clone() as i8),
                    );
                }
                BlockStateValue::Int {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::Int(serialized_value.clone()),
                    );
                }
                BlockStateValue::Enum {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::String(serialized_value.clone()),
                    );
                }
            }
        }
        let mut tag: HashMap<String, nbtx::Value> = HashMap::new();
        tag.insert(String::from("name"), nbtx::Value::String(identifier));
        tag.insert(String::from("states"), nbtx::Value::Compound(states));
        tag.insert(
            String::from("version"),
            nbtx::Value::Int(*BLOCK_STATE_VERSION),
        );
        tag
    }

    fn get_new_block_state(
        &self,
        properties: BlockType,
        values: Vec<BlockStateValue>,
    ) -> Option<BlockPermutation> {
        let bits: u8 = properties.get_special_value_bits();
        match (bits <= 16) {
            true => {
                properties.get_block_permutation(Self::compute_special_value(values, Some(bits)))
            }
            false => None,
        }
    }
}
