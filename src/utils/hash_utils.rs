#[allow(non_snake_case)]
pub mod HashUtils {
    use crate::level::block::property::r#type::block_property_type::BlockPropertyTypeTrait;
    use crate::level::block::property::value::block_property_value::{BlockPropertyValue, BlockPropertyValueTrait};
    use std::collections::HashMap;

    pub fn compute_block_state_hash(identifier: String, property_values: Vec<BlockPropertyValue>) -> i32 {
        if (identifier == "minecraft:unknown") { return -2; }
        
        let mut states: HashMap<String, nbtx::Value> = HashMap::new();
        for val in property_values {
            match val {
                BlockPropertyValue::Boolean(value) => {
                    states.insert(value.get_property_type().get_name(), nbtx::Value::Byte(value.get_serialized_value()));
                }
                BlockPropertyValue::Int(value) => {
                    states.insert(value.get_property_type().get_name(), nbtx::Value::Int(value.get_serialized_value()));
                }
                BlockPropertyValue::Enum(value) => {
                    states.insert(value.get_property_type().get_name(), nbtx::Value::String(value.get_serialized_value()));
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
            use std::collections::{BTreeMap, HashMap};
            use serde::{Serialize, Serializer};
            use serde::ser::SerializeMap;

            const FNV1A_32_INIT: i32 = -0x7ee3623b;
            const FNV1A_32_PRIME: i32 = 0x01000193;
            
            pub struct SortedCompound {
                compound: HashMap<String, nbtx::Value>
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
                    let map: BTreeMap<String, nbtx::Value> = self.compound.clone().into_iter().collect(); 
                    
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