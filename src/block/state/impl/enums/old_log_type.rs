use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OldLogType {
    Oak,
    Spruce,
    Birch,
    Jungle,
}
