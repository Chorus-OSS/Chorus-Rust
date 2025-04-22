use strum_macros::{Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ChemistryTableType {
    CompoundCreator,
    ElementConstructor,
    LabTable,
    MaterialReducer,
}
