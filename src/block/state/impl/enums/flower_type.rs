use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum FlowerType {
    Poppy,
    Orchid,
    Allium,
    Houstonia,
    TulipRed,
    TulipOrange,
    TulipWhite,
    TulipPink,
    Oxeye,
    Cornflower,
    LilyOfTheValley,
}
