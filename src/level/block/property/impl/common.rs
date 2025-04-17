use crate::level::block::property::r#type::boolean_property_type::BooleanPropertyType;
use crate::level::block::property::r#type::enum_property_type::EnumPropertyType;
use crate::level::block::property::r#type::int_property_type::IntPropertyType;
use crate::level::block::property::r#impl::enums::attachment::Attachment;
use crate::level::block::property::r#impl::enums::bamboo_leaf_size::BambooLeafSize;
use crate::level::block::property::r#impl::enums::bamboo_stalk_thickness::BambooStalkThickness;
use crate::level::block::property::r#impl::enums::big_dripleaf_tilt::BigDripleafTilt;
use crate::level::block::property::r#impl::enums::cauldron_liquid::CauldronLiquid;
use crate::level::block::property::r#impl::enums::chemistry_table_type::ChemistryTableType;
use crate::level::block::property::r#impl::enums::chisel_type::ChiselType;
use crate::level::block::property::r#impl::enums::color::Color;
use crate::level::block::property::r#impl::enums::coral_color::CoralColor;
use crate::level::block::property::r#impl::enums::cracked_state::CrackedState;
use crate::level::block::property::r#impl::enums::creaking_heart_state::CreakingHeartState;
use crate::level::block::property::r#impl::enums::damage::Damage;
use crate::level::block::property::r#impl::enums::dirt_type::DirtType;
use crate::level::block::property::r#impl::enums::double_plant_type::DoublePlantType;
use crate::level::block::property::r#impl::enums::dripstone_thickness::DripstoneThickness;
use crate::level::block::property::r#impl::enums::level_direction::LeverDirection;
use crate::level::block::property::r#impl::enums::minecraft_cardinal_direction::MinecraftCardinalDirection;
use crate::level::block::property::r#impl::enums::minecraft_vertical_half::MinecraftVerticalHalf;
use crate::level::block::property::r#impl::enums::monster_egg_stone_type::MonsterEggStoneType;
use crate::level::block::property::r#impl::enums::new_leaf_type::NewLeafType;
use crate::level::block::property::r#impl::enums::old_leaf_type::OldLeafType;
use crate::level::block::property::r#impl::enums::orientation::Orientation;
use crate::level::block::property::r#impl::enums::pale_moss_carpet_side::PaleMossCarpetSide;
use crate::level::block::property::r#impl::enums::portal_axis::PortalAxis;
use crate::level::block::property::r#impl::enums::prismarine_block_type::PrismarineBlockType;
use crate::level::block::property::r#impl::enums::sand_type::SandType;
use crate::level::block::property::r#impl::enums::sea_grass_type::SeaGrassType;
use crate::level::block::property::r#impl::enums::sponge_type::SpongeType;
use crate::level::block::property::r#impl::enums::stone_brick_type::StoneBrickType;
use crate::level::block::property::r#impl::enums::stone_slab_type::StoneSlabType;
use crate::level::block::property::r#impl::enums::stone_slab_type_2::StoneSlabType2;
use crate::level::block::property::r#impl::enums::stone_slab_type_3::StoneSlabType3;
use crate::level::block::property::r#impl::enums::stone_slab_type_4::StoneSlabType4;
use crate::level::block::property::r#impl::enums::structure_block_type::StructureBlockType;
use crate::level::block::property::r#impl::enums::structure_void_type::StructureVoidType;
use crate::level::block::property::r#impl::enums::tall_grass_type::TallGrassType;
use crate::level::block::property::r#impl::enums::torch_facing_direction::TorchFacingDirection;
use crate::level::block::property::r#impl::enums::turtle_egg_count::TurtleEggCount;
use crate::level::block::property::r#impl::enums::vault_state::VaultState;
use crate::level::block::property::r#impl::enums::wall_block_type::WallBlockType;
use crate::level::block::property::r#impl::enums::wall_connection_type::WallConnectionType;
use crate::level::block::property::r#impl::enums::wood_type::WoodType;
use crate::math::enums::axis::Axis;
use crate::math::enums::block_face::BlockFace;
use once_cell::sync::Lazy;
use strum::VariantNames;

pub const ACTIVE: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::str_new(
        "active", 
        false
    )
);

pub const AGE_16: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::str_new(
        "age", 
        0, 
        15, 
        0
    )
);

pub const AGE_6: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::str_new(
        "age", 
        0, 
        5, 
        0
    )
);

pub const AGE_4: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::str_new(
        "age", 
        0, 
        3, 
        0
    )
);

pub const AGE_3: Lazy<IntPropertyType> = Lazy::new(|| 
    IntPropertyType::str_new(
        "age", 
        0, 
        2, 
        0
    )
);

pub const AGE_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::str_new(
        "age_bit", 
        false
    )
);

pub const ALLOW_UNDERWATER_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::str_new(
        "allow_underwater_bit", 
        false
    )
);

pub const ATTACHED_BIT: Lazy<BooleanPropertyType> = Lazy::new(|| 
    BooleanPropertyType::str_new(
        "attached_bit", 
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

pub const BAMBOO_STALK_THICKNESS: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "bamboo_stalk_thickness",
        BambooStalkThickness::VARIANTS,
        BambooStalkThickness::VARIANTS[0]
    )
);

pub const BIG_DRIPLEAF_HEAD: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "big_dripleaf_head",
        false
    )
);

pub const BIG_DRIPLEAF_TILT: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "big_dripleaf_tilt",
        BigDripleafTilt::VARIANTS,
        BigDripleafTilt::VARIANTS[0]
    )
);

pub const BITE_COUNTER: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "bite_counter",
        0,
        6,
        0
    )
);

pub const BLOCK_LIGHT_LEVEL: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "block_light_level",
        0,
        15,
        0
    )
);

pub const BLOOM: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "bloom",
        false
    )
);

pub const BOOKS_STORED: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "books_stored",
        0,
        63,
        0,
    )
);

pub const BREWING_STAND_SLOT_A_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "brewing_stand_slot_a_bit",
        false
    )
);

pub const BREWING_STAND_SLOT_B_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "brewing_stand_slot_b_bit",
        false
    )
);

pub const BREWING_STAND_SLOT_C_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "brewing_stand_slot_c_bit",
        false
    )
);

pub const BRUSHED_PROGRESS: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "brushed_progress",
        0,
        3,
        0
    )
);

pub const BUTTON_PRESSED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "button_pressed_bit",
        false
    )
);

pub const CAN_SUMMON: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "can_summon",
        false
    )
);

pub const CANDLES: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "candles",
        0,
        3,
        0
    )
);

pub const CAULDRON_LIQUID: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "cauldron_liquid",
        CauldronLiquid::VARIANTS,
        CauldronLiquid::VARIANTS[0]
    )
);

pub const CHEMISTRY_TABLE_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "chemistry_table_type",
        ChemistryTableType::VARIANTS,
        ChemistryTableType::VARIANTS[0]
    )
);

pub const CHISEL_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "chisel_type",
        ChiselType::VARIANTS,
        ChiselType::VARIANTS[0]
    )
);

pub const CLUSTER_COUNT: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "cluster_count",
        0,
        3,
        0
    )
);

pub const COLOR: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "color",
        Color::VARIANTS,
        Color::VARIANTS[0]
    )
);

pub const COLOR_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "color_bit",
        false
    )
);

pub const COMPOSTER_FILL_LEVEL: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "composter_fill_level",
        0,
        8,
        0
    )
);

pub const CONDITIONAL_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "conditional_bit",
        false
    )
);

pub const CORAL_DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "coral_direction",
        0,
        3,
        0
    )
);

pub const CORAL_FAN_DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "coral_fan_direction",
        0,
        1,
        0
    )
);

pub const CORAL_HANG_TYPE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "coral_hang_type_bit",
        false
    )
);

pub const COVERED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "covered_bit",
        false
    )
);

pub const CRACKED_STATE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "cracked_state",
        CrackedState::VARIANTS,
        CrackedState::VARIANTS[0]
    )
);

pub const CRAFTING: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "crafting",
        false
    )
);

pub const CREAKING_HEART_STATE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "creaking_heart_state",
        CreakingHeartState::VARIANTS,
        CreakingHeartState::VARIANTS[0]
    )
);

pub const DAMAGE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "damage",
        Damage::VARIANTS,
        Damage::VARIANTS[0]
    )
);

pub const DEAD_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "dead_bit",
        false
    )
);

pub const DEPRECATED: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "deprecated",
        0,
        3,
        0
    )
);

pub const DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "direction",
        0,
        3,
        0
    )
);

pub const DIRT_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "dirt_type",
        DirtType::VARIANTS,
        DirtType::VARIANTS[0]
    )
);

pub const DISARMED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "disarmed_bit",
        false
    )
);

pub const DOOR_HINGE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "door_hinge_bit",
        false
    )
);

pub const DOUBLE_PLANT_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "double_plant_type",
        DoublePlantType::VARIANTS,
        DoublePlantType::VARIANTS[0]
    )
);

pub const DRAG_DOWN: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "drag_down",
        false
    )
);

pub const DRIPSTONE_THICKNESS: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "dripstone_thickness",
        DripstoneThickness::VARIANTS,
        DripstoneThickness::VARIANTS[0]
    )
);

pub const END_PORTAL_EYE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "end_portal_eye_bit",
        false
    )
);

pub const EXPLODE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "explode_bit",
        false
    )
);

pub const EXTINGUISHED: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "extinguished",
        false,
    )
);

pub const CORAL_COLOR: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "coral_color",
        CoralColor::VARIANTS,
        CoralColor::VARIANTS[0]
    )
);

pub const FACING_DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "facing_direction",
        0,
        5,
        0
    )
);

pub const FILL_LEVEL: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "fill_level",
        0,
        6,
        0
    )
);

pub const GROUND_SIGN_DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "ground_sign_direction",
        0,
        15,
        0
    )
);

pub const GROWING_PLANT_AGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "growing_plant_age",
        0,
        25,
        0
    )
);

pub const GROWTH: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "growth",
        0,
        7,
        0
    )
);

pub const HANGING: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "hanging",
        false
    )
);

pub const HEAD_PIECE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "head_piece_bit",
        false
    )
);

pub const HEIGHT: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "height",
        0,
        7,
        0,
        
    )
);

pub const HONEY_LEVEL: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "honey_level",
        0,
        5,
        0
    )
);

pub const HUGE_MUSHROOM_BITS: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "huge_mushroom_bits",
        0,
        15,
        0
    )
);

pub const IN_WALL_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "in_wall_bit",
        false
    )
);

pub const INFINIBURN_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "infiniburn_bit",
        false
    )
);

pub const ITEM_FRAME_MAP_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "item_frame_map_bit",
        false
    )
);

pub const ITEM_FRAME_PHOTO_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "item_frame_photo_bit",
        false
    )
);

pub const KELP_AGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "kelp_age",
        0,
        25,
        0
    )
);

pub const LEVER_DIRECTION: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "lever_direction",
        LeverDirection::VARIANTS,
        LeverDirection::VARIANTS[0]
    )
);

pub const LIQUID_DEPTH: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "liquid_depth",
        0,
        15,
        0
    )
);

pub const LIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "lit",
        false
    )
);

pub const MINECRAFT_BLOCK_FACE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "minecraft:block_face",
        BlockFace::VARIANTS,
        BlockFace::VARIANTS[0]
    )
);

pub const MINECRAFT_CARDINAL_DIRECTION: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "minecraft:cardinal_direction",
        MinecraftCardinalDirection::VARIANTS,
        MinecraftCardinalDirection::VARIANTS[0]
    )
);

pub const MINECRAFT_FACING_DIRECTION: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "minecraft:facing_direction",
        BlockFace::VARIANTS,
        BlockFace::VARIANTS[0]
    )
);

pub const MINECRAFT_VERTICAL_HALF: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "minecraft:vertical_half",
        MinecraftVerticalHalf::VARIANTS,
        MinecraftVerticalHalf::VARIANTS[0]
    )
);

pub const MOISTURIZED_AMOUNT: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "moisturized_amount",
        0,
        7,
        0
    )
);

pub const MONSTER_EGG_STONE_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "monster_egg_stone_type",
        MonsterEggStoneType::VARIANTS,
        MonsterEggStoneType::VARIANTS[0]
    )
);

pub const MULTI_FACE_DIRECTION_BITS: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "multi_face_direction_bits",
        0, 
        63,
        0
    )
);

pub const NEW_LEAF_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "new_leaf_type",
        NewLeafType::VARIANTS,
        NewLeafType::VARIANTS[0]
    )
);

pub const OCCUPIED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "occupied_bit",
        false
    )
);

pub const OLD_LEAF_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "old_leaf_type",
        OldLeafType::VARIANTS,
        OldLeafType::VARIANTS[0]
    )
);

pub const OPEN_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "open_bit",
        false
    )
);

pub const ORIENTATION: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "orientation",
        Orientation::VARIANTS,
        Orientation::VARIANTS[0]
    )
);

pub const OUTPUT_LIT_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "output_lit_bit",
        false
    )
);

pub const OUTPUT_SUBTRACT_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "output_subtract_bit",
        false
    )
);

pub const PERSISTENT_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "persistent_bit",
        false
    )
);

pub const PILLAR_AXIS: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "pillar_axis",
        Axis::VARIANTS,
        Axis::VARIANTS[0]
    )
);

pub const PORTAL_AXIS: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "portal_axis",
        PortalAxis::VARIANTS,
        PortalAxis::VARIANTS[0]
    )
);

pub const POWERED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "powered_bit",
        false
    )
);

pub const PRISMARINE_BLOCK_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "prismarine_block_type",
        PrismarineBlockType::VARIANTS,
        PrismarineBlockType::VARIANTS[0]
    )
);

pub const PROPAGULE_STAGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "propagule_stage",
        0,
        4,
        0
    )
);

pub const RAIL_DATA_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "rail_data_bit",
        false
    )
);

pub const RAIL_DIRECTION_10: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "rail_direction_10",
        0,
        9,
        0
    )
);

pub const RAIL_DIRECTION_6: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "rail_direction_6",
        0,
        5,
        0
    )
);

pub const REDSTONE_SIGNAL: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "redstone_signal",
        0,
        15,
        0
    )
);

pub const REPEATER_DELAY: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "repeater_delay",
        0,
        3,
        0
    )
);

pub const RESPAWN_ANCHOR_CHARGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "respawn_anchor_charge",
        0,
        4,
        0
    )
);

pub const ROTATION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "rotation",
        0,
        3,
        0
    )
);

pub const SAND_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "sand_type",
        SandType::VARIANTS,
        SandType::VARIANTS[0]
    )
);

pub const SCULK_SENSOR_PHASE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "sculk_sensor_phase",
        0,
        2,
        0
    )
);

pub const SEA_GRASS_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "sea_grass_type",
        SeaGrassType::VARIANTS,
        SeaGrassType::VARIANTS[0]
    )
);

pub const SPONGE_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "sponge_type",
        SpongeType::VARIANTS,
        SpongeType::VARIANTS[0]
    )
);

pub const STABILITY: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "stability",
        0,
        7,
        0
    )
);

pub const STABILITY_CHECK: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "stability_check",
        false
    )
);

pub const STONE_BRICK_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "stone_brick_type",
        StoneBrickType::VARIANTS,
        StoneBrickType::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "stone_slab_type",
        StoneSlabType::VARIANTS,
        StoneSlabType::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_2: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "stone_slab_type_2",
        StoneSlabType2::VARIANTS,
        StoneSlabType2::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_3: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "stone_slab_type_3",
        StoneSlabType3::VARIANTS,
        StoneSlabType3::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_4: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "stone_slab_type_4",
        StoneSlabType4::VARIANTS,
        StoneSlabType4::VARIANTS[0]
    )
);

pub const STRIPPED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "stripped_bit",
        false
    )
);

pub const STRUCTURE_BLOCK_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "structure_block_type",
        StructureBlockType::VARIANTS,
        StructureBlockType::VARIANTS[0]
    )
);

pub const STRUCTURE_VOID_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "structure_void_type",
        StructureVoidType::VARIANTS,
        StructureVoidType::VARIANTS[0]
    )
);

pub const SUSPENDED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "suspended_bit",
        false
    )
);

pub const TALL_GRASS_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "tall_grass_type",
        TallGrassType::VARIANTS,
        TallGrassType::VARIANTS[0]
    )
);

pub const TOGGLE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "toggle_bit",
        false
    )
);

pub const TORCH_FACING_DIRECTION: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "torch_facing_direction",
        TorchFacingDirection::VARIANTS,
        TorchFacingDirection::VARIANTS[0]
    )
);

pub const TRIGGERED_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "triggered_bit",
        false
    )
);

pub const TURTLE_EGG_COUNT: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "turtle_egg_count",
        TurtleEggCount::VARIANTS,
        TurtleEggCount::VARIANTS[0]
    )
);

pub const TWISTING_VINES_AGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "twisting_vines_age",
        0,
        25,
        0
    )
);

pub const UPDATE_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "update_bit",
        false
    )
);

pub const UPPER_BLOCK_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "upper_block_bit",
        false
    )
);

pub const UPSIDE_DOWN_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "upside_down_bit",
        false
    )
);

pub const VINE_DIRECTION_BITS: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "vine_direction_bits",
        0,
        15,
        0
    )
);

pub const WALL_BLOCK_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wall_block_type",
        WallBlockType::VARIANTS,
        WallBlockType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_EAST: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wall_connection_type_east",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_NORTH: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wall_connection_type_north",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_SOUTH: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wall_connection_type_south",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_WEST: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wall_connection_type_west",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_EAST: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "pale_moss_carpet_side_east",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_NORTH: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "pale_moss_carpet_side_north",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_SOUTH: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "pale_moss_carpet_side_south",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_WEST: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "pale_moss_carpet_side_west",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const TIP: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "tip",
        false
    )
);

pub const NATURAL: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "natural",
        false
    )
);

pub const WALL_POST_BIT: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "wall_post_bit",
        false
    )
);

pub const WEEPING_VINES_AGE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "weeping_vines_age",
        0,
        25,
        0,
    )
);

pub const WEIRDO_DIRECTION: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "weirdo_direction",
        0,
        3,
        0
    )
);

pub const WOOD_TYPE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "wood_type",
        WoodType::VARIANTS,
        WoodType::VARIANTS[0]
    )
);

pub const TRIAL_SPAWNER_STATE: Lazy<IntPropertyType> = Lazy::new(||
    IntPropertyType::str_new(
        "trial_spawner_state",
        0,
        5,
        0
    )
);

pub const VAULT_STATE: Lazy<EnumPropertyType> = Lazy::new(||
    EnumPropertyType::str_new(
        "vault_state",
        VaultState::VARIANTS,
        VaultState::VARIANTS[0]
    )
);

pub const OMINOUS: Lazy<BooleanPropertyType> = Lazy::new(||
    BooleanPropertyType::str_new(
        "ominous",
        false
    )
);