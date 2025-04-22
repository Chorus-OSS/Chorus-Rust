use crate::level::bit_array::bit_array_version::BitArrayVersion;
use crate::level::bit_array::padded_bit_array::PaddedBitArray;
use crate::level::bit_array::pow2_bit_array::Pow2BitArray;
use crate::level::bit_array::singleton_bit_array::SingletonBitArray;

pub trait BitArrayTrait {
    fn set(&mut self, index: usize, value: i32);
    fn get(&self, index: usize) -> i32;

    fn get_size(&self) -> usize;
    fn get_words(&self) -> Vec<i32>;
    fn get_version(&self) -> &BitArrayVersion;
}

pub enum BitArray {
    PaddedBitArray(PaddedBitArray),
    Pow2BitArray(Pow2BitArray),
    SingletonBitArray(SingletonBitArray),
}

impl BitArrayTrait for BitArray {
    fn set(&mut self, index: usize, value: i32) {
        match self {
            BitArray::PaddedBitArray(p) => p.set(index, value),
            BitArray::Pow2BitArray(p) => p.set(index, value),
            BitArray::SingletonBitArray(p) => p.set(index, value),
        }
    }

    fn get(&self, index: usize) -> i32 {
        match self {
            BitArray::PaddedBitArray(p) => p.get(index),
            BitArray::Pow2BitArray(p) => p.get(index),
            BitArray::SingletonBitArray(p) => p.get(index),
        }
    }

    fn get_size(&self) -> usize {
        match self {
            BitArray::PaddedBitArray(p) => p.get_size(),
            BitArray::Pow2BitArray(p) => p.get_size(),
            BitArray::SingletonBitArray(p) => p.get_size(),
        }
    }

    fn get_words(&self) -> Vec<i32> {
        match self {
            BitArray::PaddedBitArray(p) => p.get_words(),
            BitArray::Pow2BitArray(p) => p.get_words(),
            BitArray::SingletonBitArray(p) => p.get_words(),
        }
    }

    fn get_version(&self) -> &BitArrayVersion {
        match self {
            BitArray::PaddedBitArray(p) => p.get_version(),
            BitArray::Pow2BitArray(p) => p.get_version(),
            BitArray::SingletonBitArray(p) => p.get_version(),
        }
    }
}
