use strum_macros::{Display, EnumString, VariantNames};
use crate::math::enums::block_face::BlockFace;

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TorchFacingDirection {
    Unknown,
    West,
    East,
    North,
    South,
    Top,
}

impl TorchFacingDirection {
    pub fn get_direction(&self) -> Option<BlockFace> {
        match self {
            TorchFacingDirection::Unknown => None,
            TorchFacingDirection::West => Some(BlockFace::East),
            TorchFacingDirection::East => Some(BlockFace::West),
            TorchFacingDirection::North => Some(BlockFace::South),
            TorchFacingDirection::South => Some(BlockFace::North),
            TorchFacingDirection::Top => Some(BlockFace::Up)
        }
    }
    
    pub fn get_attached_face(&self) -> BlockFace {
        match self {
            TorchFacingDirection::East => BlockFace::East,
            TorchFacingDirection::West => BlockFace::West,
            TorchFacingDirection::South => BlockFace::South,
            TorchFacingDirection::North => BlockFace::North,
            _ => BlockFace::Down,
        }
    }
    
    pub fn from_direction(direction: BlockFace) -> Self {
        match direction {
            BlockFace::Up => TorchFacingDirection::Top,
            BlockFace::East => TorchFacingDirection::West,
            BlockFace::West => TorchFacingDirection::East,
            BlockFace::South => TorchFacingDirection::North,
            BlockFace::North => TorchFacingDirection::South,
            _ => TorchFacingDirection::Unknown,
        }
    }
    
    pub fn from_attached_face(attached_face: BlockFace) -> Self {
        match attached_face {
            BlockFace::Down=> TorchFacingDirection::Top,
            BlockFace::South => TorchFacingDirection::South,
            BlockFace::North => TorchFacingDirection::North,
            BlockFace::East => TorchFacingDirection::East,
            BlockFace::West => TorchFacingDirection::West,
            _ => TorchFacingDirection::Unknown,
        }
    }
}