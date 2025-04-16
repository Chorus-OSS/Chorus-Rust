use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Black,
    Blue,
    Brown,
    Cyan,
    Gray,
    Green,
    LightBlue,
    Lime,
    Magenta,
    Orange,
    Pink,
    Purple,
    Red,
    Silver,
    White,
    Yellow
}