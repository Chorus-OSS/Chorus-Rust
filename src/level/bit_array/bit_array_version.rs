use crate::level::bit_array::bit_array::{BitArray, BitArrayTrait};
use crate::level::bit_array::padded_bit_array::PaddedBitArray;
use crate::level::bit_array::pow2_bit_array::Pow2BitArray;
use crate::level::bit_array::singleton_bit_array::SingletonBitArray;

#[derive(Clone, PartialEq, Eq)]
pub struct BitArrayVersion {
    pub bits: u8,
    pub entries_per_word: u8,
    pub next: Option<&'static BitArrayVersion>,
}

impl BitArrayVersion {
    pub const V16: BitArrayVersion = BitArrayVersion { bits: 16, entries_per_word: 2, next: None };
    pub const V8: BitArrayVersion = BitArrayVersion { bits: 8, entries_per_word: 4, next: Some(&Self::V16) };
    pub const V6: BitArrayVersion = BitArrayVersion { bits: 6, entries_per_word: 5, next: Some(&Self::V8) };
    pub const V5: BitArrayVersion = BitArrayVersion { bits: 5, entries_per_word: 6, next: Some(&Self::V6) };
    pub const V4: BitArrayVersion = BitArrayVersion { bits: 4, entries_per_word: 8, next: Some(&Self::V5) };
    pub const V3: BitArrayVersion = BitArrayVersion { bits: 3, entries_per_word: 10, next: Some(&Self::V4) };
    pub const V2: BitArrayVersion = BitArrayVersion { bits: 2, entries_per_word: 16, next: Some(&Self::V3) };
    pub const V1: BitArrayVersion = BitArrayVersion { bits: 1, entries_per_word: 32, next: Some(&Self::V2) };
    pub const V0: BitArrayVersion = BitArrayVersion { bits: 0, entries_per_word: 2, next: Some(&Self::V1) };

    pub const VALUES: [&'static BitArrayVersion; 9] = [
        &Self::V16,
        &Self::V8,
        &Self::V6,
        &Self::V5,
        &Self::V4,
        &Self::V3,
        &Self::V2,
        &Self::V1, 
        &Self::V0
    ];
    
    pub fn get_words_for_size(&self, size: usize) -> i32 {
        (size as f32 / self.entries_per_word as f32).ceil() as i32
    }
    
    pub fn get_max_entry_value(&self) -> i32 {
        (1 << self.bits) - 1
    }

    pub fn create_array(&self, size: usize, mut words: Option<Vec<i32>>) -> BitArray {
        let words = words.unwrap_or(vec![0; self.get_words_for_size(size) as usize]);
        
        match self {
            &BitArrayVersion::V3 | 
            &BitArrayVersion::V5 | 
            &BitArrayVersion::V6 => {
                BitArray::PaddedBitArray(PaddedBitArray::new(
                    self.clone(),
                    size,
                    words
                ))
            }
            
            &BitArrayVersion::V0 => {
                BitArray::SingletonBitArray(SingletonBitArray::new())
            }
            
            _ => {
                BitArray::Pow2BitArray(Pow2BitArray::new(
                    self.clone(),
                    size,
                    words,
                ))
            }
        }
    }

    pub fn get(version: u8, read: bool) -> Option<&'static BitArrayVersion> {
        for ver in Self::VALUES.iter() {
            if (!read && ver.entries_per_word <= version) ||
                (read && ver.bits == version) 
            { 
                return Some(ver)
            }
        }
        
        None
    }
}