use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum StoneSlabType3 {
    EndStoneBrick,
    SmoothRedSandstone,
    PolishedAndesite,
    Andesite,
    Diorite,
    PolishedDiorite,
    Granite,
    PolishedGranite,
}
