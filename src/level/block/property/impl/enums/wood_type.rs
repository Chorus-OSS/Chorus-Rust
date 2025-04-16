use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum WoodType {
    Oak,
    Spruce,
    Birch,
    Jungle,
    Acacia,
    DarkOak,
    Cherry,
    PaleOak,
    Mangrove,
}