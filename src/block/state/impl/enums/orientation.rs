use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    EastUp,
    NorthUp,
    SouthUp,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
}
