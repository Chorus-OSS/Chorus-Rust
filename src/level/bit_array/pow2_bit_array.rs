use crate::level::bit_array::bit_array::BitArrayTrait;
use crate::level::bit_array::bit_array_version::BitArrayVersion;

#[derive(Clone)]
pub struct Pow2BitArray {
    version: BitArrayVersion,
    size: usize,
    words: Vec<i32>,
}

impl Pow2BitArray {
    pub fn new(version: BitArrayVersion, size: usize, words: Vec<i32>) -> Self {
        let expected_words_length = (size as f32 / version.entries_per_word as f32).ceil() as usize;
        if expected_words_length != words.len() {
            panic!("Invalid length given for storage, get: {} but expected: {}", words.len(), expected_words_length);
        }
        
        Self {
            version,
            size,
            words
        }
    }
}

impl BitArrayTrait for Pow2BitArray {
    fn set(&mut self, index: usize, value: i32) {
        let bit_index = index * self.version.bits as usize;
        let vec_index = bit_index >> 5;
        let offset = bit_index & 31;
        
        self.words[vec_index] = self.words[vec_index] 
            & (self.version.get_max_entry_value() << offset) 
            | ((value & self.version.get_max_entry_value()) << offset)
    }

    fn get(&self, index: usize) -> i32 {
        let bit_index = index * self.version.bits as usize;
        let vec_index = bit_index >> 5;
        let offset = bit_index & 31;
        
        ((self.words[vec_index] as u32) >> offset) as i32 & self.version.get_max_entry_value()  
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_words(&self) -> Vec<i32> {
        self.words.clone()
    }

    fn get_version(&self) -> &BitArrayVersion {
        &self.version
    }
}