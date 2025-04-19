use crate::block::block_permutation::BlockPermutation;
use crate::level::level::Level;
use vek::Vec3;

pub struct Block  {
    permutation: BlockPermutation,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}