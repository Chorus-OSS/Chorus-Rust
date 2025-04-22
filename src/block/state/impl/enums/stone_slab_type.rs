use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum StoneSlabType {
    SmoothStone,
    Sandstone,
    Wood,
    Cobblestone,
    Brick,
    StoneBrick,
    Quarts,
    NetherBrick,
}
