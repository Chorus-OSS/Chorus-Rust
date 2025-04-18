use crate::block::block::TBlock;

pub trait TBlockTransparent: TBlock {}

pub trait AsTBlockTransparent {
    fn as_block_transparent(&self) -> Option<&dyn TBlockTransparent>;
}

impl<T: TBlockTransparent> AsTBlockTransparent for T {
    fn as_block_transparent(&self) -> Option<&dyn TBlockTransparent> {
        Some(self)
    }
}