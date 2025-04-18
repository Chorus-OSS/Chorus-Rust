use crate::block::block::TBlock;
use crate::level::block::block_properties::TBlockProperties;
use crate::level::block::block_state::BlockState;
use crate::level::block::block_states::BlockStates;
use crate::level::level::Level;
use vek::Vec3;

pub trait TBlockTransparent: TBlock {
    fn get_state(&self) -> &BlockState;
    fn get_position(&self) -> &Vec3<i32>;
    fn get_layer(&self) -> &i32;
    fn get_level(&self) -> &Level;

    fn get_states() -> &'static BlockStates where Self: Sized;
    fn get_default_state() -> &'static BlockState where Self: Sized;
    fn get_properties() -> &'static dyn TBlockProperties where Self: Sized;
}

impl<T: TBlockTransparent> TBlock for T {
    fn get_state(&self) -> &BlockState {
        TBlockTransparent::get_state(self)
    }

    fn get_position(&self) -> &Vec3<i32> {
        TBlockTransparent::get_position(self)
    }

    fn get_layer(&self) -> &i32 {
        TBlockTransparent::get_layer(self)
    }

    fn get_level(&self) -> &Level {
        TBlockTransparent::get_level(self)
    }

    fn get_states() -> &'static BlockStates where Self: Sized {
        <T as TBlockTransparent>::get_states()
    }

    fn get_default_state() -> &'static BlockState where Self: Sized {
        <T as TBlockTransparent>::get_default_state()
    }

    fn get_properties() -> &'static dyn TBlockProperties where Self: Sized {
        <T as TBlockTransparent>::get_properties()
    }

    fn as_transparent_block(&self) -> Option<&dyn TBlockTransparent> {
        Some(self)
    }
}