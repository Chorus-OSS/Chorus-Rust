use crate::block::block_permutation::BlockPermutation;
use crate::block::r#impl::air::Air;
use crate::level::biome::biome_id::BiomeID;
use crate::level::bit_array::bit_array_version::BitArrayVersion;
use crate::level::palette::palette::Palette;
use bedrockrs::proto::ProtoCodec;
use bedrockrs::proto::error::ProtoCodecError;
use std::io::Cursor;
use std::sync::atomic::{AtomicI64, Ordering};

pub struct SubChunk {
    index: u8,
    block_layers: Vec<Palette<BlockPermutation>>,
    biomes: Palette<i32>,
    block_lights: Vec<u8>,
    sky_lights: Vec<u8>,
    block_changes: AtomicI64,
}

impl SubChunk {
    pub const SIZE: usize = 16 * 16 * 16;
    pub const VERSION: u8 = 9;

    pub fn new(index: u8, block_layers: Option<Vec<Palette<BlockPermutation>>>) -> Self {
        Self {
            index,
            block_layers: block_layers.unwrap_or(vec![
                Palette::new(
                    Air::TYPE.get_default_permutation().clone(),
                    Some(vec![Air::TYPE.get_default_permutation().clone(); 16]),
                    Some(BitArrayVersion::V2),
                ),
                Palette::new(
                    Air::TYPE.get_default_permutation().clone(),
                    Some(vec![Air::TYPE.get_default_permutation().clone(); 16]),
                    Some(BitArrayVersion::V2),
                ),
            ]),
            biomes: Palette::new(BiomeID::PLAINS, None, None),
            block_lights: vec![0; SubChunk::SIZE],
            sky_lights: vec![0; SubChunk::SIZE],
            block_changes: AtomicI64::new(0),
        }
    }

    pub fn get_block_permutation(
        &self,
        x: i32,
        y: i32,
        z: i32,
        layer: Option<usize>,
    ) -> &BlockPermutation {
        self.block_layers[layer.unwrap_or(0)].get(Self::index(x, y, z) as usize)
    }

    pub fn set_block_permutation(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        layer: Option<usize>,
        permutation: BlockPermutation,
    ) {
        self.block_changes.fetch_add(1, Ordering::SeqCst);
        self.block_layers[layer.unwrap_or(0)].set(Self::index(x, y, z) as usize, permutation);
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> i32 {
        *self.biomes.get(Self::index(x, y, z) as usize)
    }

    pub fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: i32) {
        self.biomes.set(Self::index(x, y, z) as usize, biome);
    }

    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> u8 {
        self.block_lights[Self::index(x, y, z) as usize]
    }

    pub fn set_block_light(&mut self, x: i32, y: i32, z: i32, light: u8) {
        self.block_lights[Self::index(x, y, z) as usize] = light;
    }

    pub fn get_sky_light(&self, x: i32, y: i32, z: i32) -> u8 {
        self.sky_lights[Self::index(x, y, z) as usize]
    }

    pub fn set_sky_light(&mut self, x: i32, y: i32, z: i32, light: u8) {
        self.sky_lights[Self::index(x, y, z) as usize] = light;
    }

    pub fn is_empty(&self) -> bool {
        for block_layer in self.block_layers.iter() {
            if !block_layer.is_empty() || block_layer.get(0) != Air::TYPE.get_default_permutation()
            {
                return false;
            }
        }
        true
    }

    fn index(x: i32, y: i32, z: i32) -> i32 {
        (x << 8) + (z << 4) + y
    }
}

impl ProtoCodec for SubChunk {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Self::VERSION.proto_serialize(stream)?;

        let num_layers = self.block_layers.len().min(u8::MAX as usize) as u8;
        num_layers.proto_serialize(stream)?;
        self.index.proto_serialize(stream)?;

        for i in 0..num_layers {
            self.block_layers[i as usize].proto_serialize(stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let _ = u8::proto_deserialize(stream)?; // version, UNUSED, but should always be 9.
        let num_layers = u8::proto_deserialize(stream)?;
        let index = u8::proto_deserialize(stream)?;

        let mut layers = Vec::with_capacity(num_layers as usize);
        for _ in 0..num_layers {
            layers.push(<Palette<BlockPermutation>>::proto_deserialize(stream)?);
        }

        Ok(Self::new(index, Some(layers)))
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u8>()
            + size_of::<u8>()
            + size_of::<u8>()
            + self
                .block_layers
                .iter()
                .take(u8::MAX as usize)
                .map(|v| v.get_size_prediction())
                .sum::<usize>()
    }
}
