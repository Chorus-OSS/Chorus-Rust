use strum_macros::{Display, EnumString, VariantNames};

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
    pub fn get_direction(&self) -> Option<todo!("BlockFace")> {
        match self {
            TorchFacingDirection::Unknown => None,
            TorchFacingDirection::West => Some(todo!("BlockFace::East")),
            TorchFacingDirection::East => Some(todo!("BlockFace::West")),
            TorchFacingDirection::North => Some(todo!("BlockFace::South")),
            TorchFacingDirection::South => Some(todo!("BlockFace::North")),
            TorchFacingDirection::Top => Some(todo!("BlockFace::Up"))
        }
    }
    
    pub fn get_attached_face(&self) -> todo!("BlockFace") {
        match self {
            TorchFacingDirection::East => todo!("BlockFace::East"),
            TorchFacingDirection::West => todo!("BlockFace::West"),
            TorchFacingDirection::South => todo!("BlocKFace::South"),
            TorchFacingDirection::North => todo!("BlockFace::North"),
            _ => todo!("BlockFace::Down"),
        }
    }
    
    pub fn from_direction(direction: todo!("BlockFace")) -> Self {
        match direction {
            todo!("BlockFace::Up") => TorchFacingDirection::Top,
            todo!("BlockFace::East") => TorchFacingDirection::West,
            todo!("BlockFace::West") => TorchFacingDirection::East,
            todo!("BlockFace::South") => TorchFacingDirection::North,
            todo!("BlocKFace::North") => TorchFacingDirection::South,
            _ => TorchFacingDirection::Unknown,
        }
    }
    
    pub fn from_attached_face(attached_face: todo!("BlockFace")) -> Self {
        match attached_face {
            todo!("BlockFace::Down") => TorchFacingDirection::Top,
            todo!("BlocKFace::South") => TorchFacingDirection::South,
            todo!("BlockFace::North") => TorchFacingDirection::North,
            todo!("BlocKFace::East") => TorchFacingDirection::East,
            todo!("BlocKFace::West") => TorchFacingDirection::West,
            _ => TorchFacingDirection::Unknown,
        }
    }
}