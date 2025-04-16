use std::convert::{AsRef, Into};
use crate::level::block::property::r#impl::enums::attachment::Attachment;
use crate::level::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::level::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::level::block::property::r#type::int_property_type::IntPropertyType;
use once_cell::sync::Lazy;
use std::string::ToString;
use strum::VariantNames;
use crate::level::block::property::r#impl::enums::bamboo_leaf_size::BambooLeafSize;

pub const ACTIVE: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::new(
        String::from("active"), 
        false
    )
);

pub const AGE_16: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::new(
        String::from("age"), 
        0, 
        15, 
        0
    )
);

pub const AGE_6: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::new(
        String::from("age"), 
        0, 
        5, 
        0
    )
);

pub const AGE_4: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::new(
        String::from("age"), 
        0, 
        3, 
        0
    )
);

pub const AGE_3: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::new(
        String::from("age"), 
        0, 
        2, 
        0
    )
);

pub const AGE_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::new(
        String::from("age_bit"), 
        false
    )
);

pub const ALLOW_UNDERWATER_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::new(
        String::from("allow_underwater_bit"), 
        false
    )
);

pub const ATTACHED_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::new(
        String::from("attached_bit"), 
        false
    )
);

pub const ATTACHMENT: Lazy<EnumPropertyType> = Lazy::new(|| 
    EnumPropertyType::str_new(
        "attachment", 
        Attachment::VARIANTS,
        Attachment::VARIANTS[0]
    )
);

pub const BAMBOO_LEAF_SIZE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "bamboo_leaf_size",
        BambooLeafSize::VARIANTS,
        BambooLeafSize::VARIANTS[0]
    )
);
