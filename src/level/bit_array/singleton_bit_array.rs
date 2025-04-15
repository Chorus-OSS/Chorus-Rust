use crate::level::bit_array::bit_array::BitArrayTrait;
use crate::level::bit_array::bit_array_version::BitArrayVersion;

#[derive(Clone)]
pub struct SingletonBitArray {}

impl SingletonBitArray {
    pub fn new() -> Self {
        Self {}
    }
}

impl BitArrayTrait for SingletonBitArray {
    fn set(&mut self, _: usize, _: i32) {
        panic!("Unsupported Operation")
    }

    fn get(&self, _: usize) -> i32 {
        0
    }

    fn get_size(&self) -> usize {
        1
    }

    fn get_words(&self) -> Vec<i32> {
        vec![]
    }

    fn get_version(&self) -> &BitArrayVersion {
        &BitArrayVersion::V0
    }
}