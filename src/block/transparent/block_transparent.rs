use crate::block::block::{AsTBlockTransparent, TBlock};

pub trait TBlockTransparent: TBlock {}

impl<T: TBlockTransparent> AsTBlockTransparent for T {
    fn as_block_transparent(&self) -> Option<&dyn TBlockTransparent> {
        Some(self)
    }
}