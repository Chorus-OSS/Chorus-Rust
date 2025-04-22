use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum StoneSlabType2 {
    RedSandstone,
    Purpur,
    PrismarineRough,
    PrismarineDark,
    PrismarineBrick,
    MossyCobblestone,
    SmoothSandstone,
    RedNetherBrick,
}
