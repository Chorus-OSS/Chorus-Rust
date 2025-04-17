use strum_macros::{AsRefStr, Display, EnumString, VariantNames};

#[derive(EnumString, VariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Attachment {
    Hanging,
    Multiple,
    Side,
    Standing,
}