use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TurtleEggCount {
    OneEgg,
    TwoEgg,
    ThreeEgg,
    FourEgg,
}