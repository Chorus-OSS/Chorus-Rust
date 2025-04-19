use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum WallBlockType {
    Andesite,
    Brick,
    Cobblestone,
    Diorite,
    EndBrick,
    Granite,
    MossyCobblestone,
    MossyStoneBrick,
    NetherBrick,
    Prismarine,
    RedNetherBrick,
    RedSandstone,
    Sandstone,
    StoneBrick,
}