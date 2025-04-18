use vek::Vec3;
use crate::block::block::Block;
use crate::level::block::block_properties::BlockProperties;
use crate::level::block::block_state::BlockState;

trait BlockTransparent : Block {
    fn get_state(&self) -> &BlockState;
    fn get_position(&self) -> &Vec3<i32>;
    fn get_level(&self) -> &todo!("Level");


    fn get_properties() -> BlockProperties;
    fn get_default_state() -> BlockState;
}

impl<T> Block for T where T : BlockTransparent {
    fn get_state(&self) -> &BlockState {
        BlockTransparent::get_state(self)
    }

    fn get_position(&self) -> &Vec3<i32> {
        BlockTransparent::get_position(self)
    }

    fn get_level(&self) -> &todo!("Level") {
        BlockTransparent::get_level(self)
    }

    fn get_properties() -> BlockProperties {
        BlockTransparent::get_properties()
    }

    fn get_default_state() -> BlockState {
        BlockTransparent::get_default_state()
    }
}