use std::io::Cursor;
use bedrockrs::proto::error::ProtoCodecError;
use bedrockrs::proto::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use crate::level::bit_array::bit_array::{BitArray, BitArrayTrait};
use crate::level::bit_array::bit_array_version::BitArrayVersion;
use crate::level::block::block_state::BlockState;
use crate::level::sub_chunk::SubChunk;

pub struct Palette<V : PartialEq> {
    palette: Vec<V>,
    bit_array: BitArray,
}

impl<V : PartialEq + Clone> Palette<V> {
    pub fn new(first: V, palette: Option<Vec<V>>, version: Option<BitArrayVersion>) -> Self {
        let version = version.unwrap_or(BitArrayVersion::V2);
        let mut palette = palette.unwrap_or(vec![]);
        
        palette.push(first);
        
        Self {
            bit_array: version.create_array(SubChunk::SIZE, None),
            palette
        }
    }
    
    pub fn get(&self, index: usize) -> &V {
        let i = self.bit_array.get(index) as usize;
        
        if i >= self.palette.len() { self.palette.first().unwrap() } else { &self.palette[i] }
    }
    
    pub fn set(&mut self, index: usize, value: V) {
        let bit = self.index_for(&value);
        self.bit_array.set(index, bit as i32);
    }
    
    pub fn is_empty(&self) -> bool {
        if self.palette.len() == 1 {
            for word in self.bit_array.get_words() {
                if word != 0 {
                    return false
                }
            }
            true
        } else { false }
    }

    fn index_for(&mut self, value: &V) -> usize {
        if let Some(index) = self.palette.iter().position(|v| *v == *value) {
            return index;
        }

        let index = self.palette.len();
        self.palette.push(value.clone());

        let version = &self.bit_array.get_version();
        if index > version.get_max_entry_value() as usize {
            if let Some(next_version) = &version.next {
                self.on_resize(next_version);
            }
        }

        index
    }
    
    fn on_resize(&mut self, version: &BitArrayVersion) {
        let mut new_bit_array = version.create_array(SubChunk::SIZE, None);
        for i in 0..SubChunk::SIZE {
            new_bit_array.set(i, self.bit_array.get(i))
        }
        self.bit_array = new_bit_array;
    }
    
    fn get_header(version: &BitArrayVersion, runtime: bool) -> u8 {
        (version.bits << 1) | if runtime { 0 } else { 1 }
    }
    
    fn get_version(header: u8) -> Option<&'static BitArrayVersion> {
        BitArrayVersion::get(header << 1, true)
    }
}

impl ProtoCodec for Palette<BlockState> {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Self::get_header(self.bit_array.get_version(), true).proto_serialize(stream)?;
        for word in self.bit_array.get_words() {
            <i32 as ProtoCodecLE>::proto_serialize(&word, stream)?;
        }

        <i32 as ProtoCodecVAR>::proto_serialize(&(self.palette.len() as i32), stream)?;
        for value in self.palette.iter() {
            <i32 as ProtoCodecVAR>::proto_serialize(&value.get_hash(), stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let bit_array_version = Self::get_version(u8::proto_deserialize(stream)?).unwrap();
        let num_words = bit_array_version.get_words_for_size(SubChunk::SIZE);

        let mut words = Vec::with_capacity(num_words as usize);
        for _ in 0..num_words {
            words.push(<i32 as ProtoCodecLE>::proto_deserialize(stream)?);
        }

        let bit_array = bit_array_version.create_array(SubChunk::SIZE, Some(words));

        let states = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let mut palette = vec![];
        for _ in 0..states {
            palette.push(todo!());
        }

        Ok(Self {
            palette,
            bit_array,
        })
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u8>()
            + self.bit_array.get_words().len() * size_of::<i32>()
            + size_of::<i32>()
            + self.palette.len() * size_of::<i32>()
    }
}

impl<V : ProtoCodec + PartialEq + Clone> ProtoCodec for Palette<V> {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Self::get_header(self.bit_array.get_version(), true).proto_serialize(stream)?;
        for word in self.bit_array.get_words() {
            <i32 as ProtoCodecLE>::proto_serialize(&word, stream)?;
        }
        
        <i32 as ProtoCodecVAR>::proto_serialize(&(self.palette.len() as i32), stream)?;
        for value in self.palette.iter() {
            value.proto_serialize(stream)?;
        }
        
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let bit_array_version = Self::get_version(u8::proto_deserialize(stream)?).unwrap();
        let num_words = bit_array_version.get_words_for_size(SubChunk::SIZE);
        
        let mut words = Vec::with_capacity(num_words as usize);
        for _ in 0..num_words {
            words.push(<i32 as ProtoCodecLE>::proto_deserialize(stream)?);
        }
        
        let bit_array = bit_array_version.create_array(SubChunk::SIZE, Some(words));
        
        let states = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let mut palette = vec![];
        for _ in 0..states {
            palette.push(V::proto_deserialize(stream)?);
        }
        
        Ok(Self {
            palette,
            bit_array,
        })
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u8>() 
            + self.bit_array.get_words().len() * size_of::<i32>() 
            + size_of::<i32>()
            + self.palette.iter().map(|v| v.get_size_prediction()).sum::<usize>()
    }
}