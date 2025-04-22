use crate::math::enums::block_face::BlockFace;
use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum LeverDirection {
    #[strum(to_string = "down_x")]
    DownEastWest,
    #[strum(to_string = "down_z")]
    DownNorthSouth,
    East,
    North,
    South,
    #[strum(to_string = "up_x")]
    UpEastWest,
    #[strum(to_string = "up_z")]
    UpNorthSouth,
    West,
}

impl LeverDirection {
    pub fn get_metadata(&self) -> i32 {
        match self {
            LeverDirection::DownEastWest => 0,
            LeverDirection::East => 1,
            LeverDirection::West => 2,
            LeverDirection::South => 3,
            LeverDirection::North => 4,
            LeverDirection::UpNorthSouth => 5,
            LeverDirection::UpEastWest => 6,
            LeverDirection::DownNorthSouth => 7,
        }
    }

    pub fn get_face(&self) -> BlockFace {
        match self {
            LeverDirection::North => BlockFace::North,
            LeverDirection::East => BlockFace::East,
            LeverDirection::South => BlockFace::South,
            LeverDirection::West => BlockFace::West,
            LeverDirection::DownEastWest | LeverDirection::DownNorthSouth => BlockFace::Down,
            LeverDirection::UpEastWest | LeverDirection::UpNorthSouth => BlockFace::Up,
        }
    }
}
