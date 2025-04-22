#[allow(non_snake_case)]
pub mod HashUtils {
    use crate::block::state::block_state_value::BlockStateValue;
    use std::collections::HashMap;

    pub fn compute_block_permutation_hash(
        identifier: String,
        property_values: Vec<BlockStateValue>,
    ) -> i32 {
        if (identifier == "minecraft:unknown") {
            return -2;
        }

        let mut states: HashMap<String, nbtx::Value> = HashMap::new();
        for val in property_values {
            match val {
                BlockStateValue::Boolean {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::Byte(serialized_value as i8),
                    );
                }
                BlockStateValue::Int {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::Int(serialized_value),
                    );
                }
                BlockStateValue::Enum {
                    property_type,
                    serialized_value,
                    ..
                } => {
                    states.insert(
                        property_type.get_name().clone(),
                        nbtx::Value::String(serialized_value),
                    );
                }
            }
        }

        let mut tag: HashMap<String, nbtx::Value> = HashMap::new();

        tag.insert(String::from("name"), nbtx::Value::String(identifier));
        tag.insert(String::from("states"), nbtx::Value::Compound(states));

        FNV::r1A_i32::hash_nbt(tag)
    }

    pub mod FNV {
        pub mod r1A_i32 {
            use serde::ser::SerializeMap;
            use serde::{Serialize, Serializer};
            use std::collections::{BTreeMap, HashMap};

            const FNV1A_32_INIT: i32 = -0x7ee3623b;
            const FNV1A_32_PRIME: i32 = 0x01000193;

            pub struct SortedCompound {
                compound: HashMap<String, nbtx::Value>,
            }

            impl SortedCompound {
                pub fn new(compound: HashMap<String, nbtx::Value>) -> Self {
                    Self { compound }
                }
            }

            impl Serialize for SortedCompound {
                fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let map: BTreeMap<String, nbtx::Value> =
                        self.compound.clone().into_iter().collect();

                    let mut map_ser = ser.serialize_map(Some(map.len()))?;
                    for (k, v) in &map {
                        match v {
                            nbtx::Value::Compound(map) => {
                                let v = &SortedCompound::new(map.clone());
                                map_ser.serialize_entry(k, v)?
                            }
                            _ => {
                                map_ser.serialize_entry(k, v)?;
                            }
                        }
                    }
                    map_ser.end()
                }
            }

            pub fn hash_nbt(compound: HashMap<String, nbtx::Value>) -> i32 {
                let sorted = SortedCompound::new(compound.clone());

                hash(nbtx::to_le_bytes(&sorted).unwrap().as_slice())
            }

            pub fn hash(data: &[u8]) -> i32 {
                let mut hash = FNV1A_32_INIT;
                for &byte in data {
                    hash ^= (byte as i32 & 0xFF);
                    hash *= FNV1A_32_PRIME;
                }
                hash
            }
        }

        pub mod r1_i64 {
            const FNV1_64_INIT: i64 = -0x340d631b7bdddcdb;
            const FNV1_64_PRIME: i64 = 1099511628211;

            pub fn hash(data: &[u8]) -> i64 {
                let mut hash = FNV1_64_INIT;
                for &byte in data {
                    hash ^= (byte as i64 & 0xFF);
                    hash *= FNV1_64_PRIME
                }
                hash
            }
        }
    }
}
