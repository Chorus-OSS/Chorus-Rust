use crate::block::block::{AsTBlockTransparent, TBlock};

pub trait TBlockTransparent: TBlock {}

impl<T: TBlockTransparent> AsTBlockTransparent for T {
    fn as_transparent_block(&self) -> Option<&dyn TBlockTransparent> {
        Some(self)
    }
}