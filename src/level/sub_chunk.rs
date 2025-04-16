use crate::level::biome::biome_id::BiomeID;
use crate::level::bit_array::bit_array_version::BitArrayVersion;
use crate::level::block::block_state::BlockState;
use crate::level::block::r#impl::state::air;
use crate::level::palette::palette::Palette;
use bedrockrs::proto::error::ProtoCodecError;
use bedrockrs::proto::ProtoCodec;
use std::io::Cursor;
use std::sync::atomic::{AtomicI64, Ordering};

pub struct SubChunk {
    index: u8,
    block_layers: Vec<Palette<BlockState>>,
    biomes: Palette<i32>,
    block_lights: Vec<u8>,
    sky_lights: Vec<u8>,
    block_changes: AtomicI64,
}

impl SubChunk {
    pub const SIZE: usize = 16 * 16 * 16;
    pub const VERSION: u8 = 9;
    
    pub fn new(index: u8, block_layers: Option<Vec<Palette<BlockState>>>) -> Self {
        Self {
            index,
            block_layers: block_layers.unwrap_or(
                vec![
                    Palette::new(
                        air::PROPERTIES.get_default_state(),
                        Some(vec![
                            air::PROPERTIES.get_default_state();
                            16
                        ]),
                        Some(BitArrayVersion::V2),
                    ),
                    Palette::new(
                        air::PROPERTIES.get_default_state(),
                        Some(vec![
                            air::PROPERTIES.get_default_state();
                            16
                        ]),
                        Some(BitArrayVersion::V2),
                    )
                ]
            ),
            biomes: Palette::new(BiomeID::PLAINS, None, None),
            block_lights: vec![0; SubChunk::SIZE],
            sky_lights: vec![0; SubChunk::SIZE],
            block_changes: AtomicI64::new(0),
        }
    }
    
    pub fn get_block_state(&self, x: usize, y: usize, z: usize, layer: Option<usize>) -> &BlockState {
        self.block_layers[layer.unwrap_or(0)].get(Self::index(x, y, z))
    }
    
    pub fn set_block_state(&mut self, x: usize, y: usize, z: usize, layer: Option<usize>, block_state: BlockState) {
        self.block_changes.fetch_add(1, Ordering::SeqCst);
        self.block_layers[layer.unwrap_or(0)].set(Self::index(x, y, z), block_state);
    }
    
    pub fn get_biome(&self, x: usize, y: usize, z: usize) -> i32 {
        *self.biomes.get(Self::index(x, y, z))
    }
    
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: i32) {
        self.biomes.set(Self::index(x, y, z), biome);
    }
    
    pub fn get_block_light(&self, x: usize, y: usize, z: usize) -> u8 {
        self.block_lights[Self::index(x, y, z)]
    }
    
    pub fn set_block_light(&mut self, x: usize, y: usize, z: usize, light: u8) {
        self.block_lights[Self::index(x, y, z)] = light;
    }
    
    pub fn get_sky_light(&self, x: usize, y: usize, z: usize) -> u8 {
        self.sky_lights[Self::index(x, y, z)]
    }
    
    pub fn set_sky_light(&mut self, x: usize, y: usize, z: usize, light: u8) {
        self.sky_lights[Self::index(x, y, z)] = light;
    }
    
    pub fn is_empty(&self) -> bool {
        for block_layer in self.block_layers.iter() {
            if !block_layer.is_empty() || *block_layer.get(0) != air::PROPERTIES.get_default_state() {
                return false;
            }
        }
        true
    }
    
    fn index(x: usize, y: usize, z: usize) -> usize {
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
            layers.push(<Palette<BlockState>>::proto_deserialize(stream)?);
        }
        
        Ok(Self::new(index, Some(layers)))
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u8>()
            + size_of::<u8>()
            + size_of::<u8>()
            + self.block_layers.iter().take(u8::MAX as usize).map(|v| v.get_size_prediction()).sum::<usize>()
    }
}