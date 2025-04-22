use crate::block::block_permutation::BlockPermutation;
use crate::block::r#impl::air::Air;
use crate::level::chunk_state::ChunkState;
use crate::level::sub_chunk::SubChunk;
use std::collections::HashMap;
use std::sync::atomic::AtomicI64;

pub struct Chunk {
    x: i32,
    z: i32,
    index: i64,
    state: ChunkState,
    sub_chunks: Vec<SubChunk>,
    height_map: [i16; 16 * 16],

    changes: AtomicI64,

    entities: HashMap<i64, shipyard::EntityId>,
    block_entities: HashMap<i64, shipyard::EntityId>,

    min_height: i32,
    max_height: i32,
}

impl Chunk {
    pub fn new(x: i32, z: i32, index: i64, state: ChunkState, sub_chunks: Vec<SubChunk>) -> Self {
        Self {
            x,
            z,
            index,
            state,
            sub_chunks,
            height_map: [0; 16 * 16],
            changes: AtomicI64::new(0),
            entities: HashMap::new(),
            block_entities: HashMap::new(),

            min_height: -64,
            max_height: 319,
        }
    }

    pub fn is_sub_chunk_empty(&self, y: usize) -> bool {
        if let Some(sub) = self.sub_chunks.get(y) {
            sub.is_empty()
        } else {
            false
        }
    }

    pub fn get_sub_chunk(&self, chunk_y: i32) -> Option<&SubChunk> {
        self.sub_chunks.get(chunk_y as usize)
    }

    pub fn get_mut_sub_chunk(&mut self, chunk_y: i32) -> Option<&mut SubChunk> {
        self.sub_chunks.get_mut(chunk_y as usize)
    }

    pub fn set_sub_chunk(&mut self, chunk_y: i32, sub: SubChunk) {
        self.sub_chunks[chunk_y as usize] = sub;
    }

    pub fn get_block_permutation(
        &self,
        x: i32,
        y: i32,
        z: i32,
        layer: Option<usize>,
    ) -> BlockPermutation {
        if let Some(sub_chunk) = self.get_sub_chunk(y << 4) {
            sub_chunk.get_block_permutation(x, y, z, layer).clone()
        } else {
            Air::TYPE.get_default_permutation().clone()
        }
    }

    pub fn set_block_permutation(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        layer: Option<usize>,
        permutation: BlockPermutation,
    ) {
        if let Some(sub_chunk) = self.get_mut_sub_chunk(y << 4) {
            sub_chunk.set_block_permutation(x, y, z, layer, permutation);
        }
    }

    pub fn get_sky_light(&self, x: i32, y: i32, z: i32) -> u8 {
        if let Some(sub_chunk) = self.get_sub_chunk(y << 4) {
            sub_chunk.get_sky_light(x, y, z)
        } else {
            0
        }
    }

    pub fn set_sky_light(&mut self, x: i32, y: i32, z: i32, light: u8) {
        if let Some(sub_chunk) = self.get_mut_sub_chunk(y << 4) {
            sub_chunk.set_sky_light(x, y, z, light);
        }
    }

    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> u8 {
        if let Some(sub_chunk) = self.get_sub_chunk(y << 4) {
            sub_chunk.get_block_light(x, y, z)
        } else {
            0
        }
    }

    pub fn set_block_light(&mut self, x: i32, y: i32, z: i32, light: u8) {
        if let Some(sub_chunk) = self.get_mut_sub_chunk(y << 4) {
            sub_chunk.set_block_light(x, y, z, light);
        }
    }

    pub fn get_height(&self, x: usize, z: usize) -> &i16 {
        &self.height_map[(z << 4) | x]
    }

    pub fn set_height(&mut self, x: usize, z: usize, height: i16) {
        self.height_map[(z << 4) | x] = height
    }

    pub fn recompute_height_map(&mut self) {
        for z in 0..16 {
            for x in 0..16 {
                self.recompute_height(x, z)
            }
        }
        todo!()
    }

    pub fn recompute_height(&mut self, x: i32, z: i32) {
        todo!()
    }

    pub fn get_highest_at(&self, x: i32, z: i32) -> i32 {
        let air_state = Air::TYPE.get_default_permutation().clone();
        for y in (self.min_height..=self.max_height).rev() {
            if self.get_block_permutation(x, y, z, None) != air_state {
                return y;
            }
        }
        self.min_height
    }
}
