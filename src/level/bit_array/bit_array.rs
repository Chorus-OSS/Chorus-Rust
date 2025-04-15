use crate::level::bit_array::bit_array_version::BitArrayVersion;

pub trait BitArray : Clone {
    fn set(&mut self, index: usize, value: i32);
    fn get(&self, index: usize) -> i32;
    
    fn get_size(&self) -> usize;
    fn get_words(&self) -> &Vec<i32>;
    fn get_version(&self) -> &BitArrayVersion;
}