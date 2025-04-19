use shipyard::Component;
use crate::block::block_permutation::BlockPermutation;
use crate::block::block_state::BlockState;
use crate::level::level::Level;
use vek::Vec3;

#[derive(Component)]
pub struct Block  {
    state: BlockState,
    position: Vec3<i32>,
    layer: i32,
    level: Level,
}