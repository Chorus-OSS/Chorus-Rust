use crate::block::property::r#impl::enums::attachment::Attachment;
use crate::block::property::r#impl::enums::bamboo_leaf_size::BambooLeafSize;
use crate::block::property::r#impl::enums::bamboo_stalk_thickness::BambooStalkThickness;
use crate::block::property::r#impl::enums::big_dripleaf_tilt::BigDripleafTilt;
use crate::block::property::r#impl::enums::cauldron_liquid::CauldronLiquid;
use crate::block::property::r#impl::enums::chemistry_table_type::ChemistryTableType;
use crate::block::property::r#impl::enums::chisel_type::ChiselType;
use crate::block::property::r#impl::enums::color::Color;
use crate::block::property::r#impl::enums::coral_color::CoralColor;
use crate::block::property::r#impl::enums::cracked_state::CrackedState;
use crate::block::property::r#impl::enums::creaking_heart_state::CreakingHeartState;
use crate::block::property::r#impl::enums::damage::Damage;
use crate::block::property::r#impl::enums::dirt_type::DirtType;
use crate::block::property::r#impl::enums::double_plant_type::DoublePlantType;
use crate::block::property::r#impl::enums::dripstone_thickness::DripstoneThickness;
use crate::block::property::r#impl::enums::level_direction::LeverDirection;
use crate::block::property::r#impl::enums::minecraft_cardinal_direction::MinecraftCardinalDirection;
use crate::block::property::r#impl::enums::minecraft_vertical_half::MinecraftVerticalHalf;
use crate::block::property::r#impl::enums::monster_egg_stone_type::MonsterEggStoneType;
use crate::block::property::r#impl::enums::new_leaf_type::NewLeafType;
use crate::block::property::r#impl::enums::old_leaf_type::OldLeafType;
use crate::block::property::r#impl::enums::orientation::Orientation;
use crate::block::property::r#impl::enums::pale_moss_carpet_side::PaleMossCarpetSide;
use crate::block::property::r#impl::enums::portal_axis::PortalAxis;
use crate::block::property::r#impl::enums::prismarine_block_type::PrismarineBlockType;
use crate::block::property::r#impl::enums::sand_type::SandType;
use crate::block::property::r#impl::enums::sea_grass_type::SeaGrassType;
use crate::block::property::r#impl::enums::sponge_type::SpongeType;
use crate::block::property::r#impl::enums::stone_brick_type::StoneBrickType;
use crate::block::property::r#impl::enums::stone_slab_type::StoneSlabType;
use crate::block::property::r#impl::enums::stone_slab_type_2::StoneSlabType2;
use crate::block::property::r#impl::enums::stone_slab_type_3::StoneSlabType3;
use crate::block::property::r#impl::enums::stone_slab_type_4::StoneSlabType4;
use crate::block::property::r#impl::enums::structure_block_type::StructureBlockType;
use crate::block::property::r#impl::enums::structure_void_type::StructureVoidType;
use crate::block::property::r#impl::enums::tall_grass_type::TallGrassType;
use crate::block::property::r#impl::enums::torch_facing_direction::TorchFacingDirection;
use crate::block::property::r#impl::enums::turtle_egg_count::TurtleEggCount;
use crate::block::property::r#impl::enums::vault_state::VaultState;
use crate::block::property::r#impl::enums::wall_block_type::WallBlockType;
use crate::block::property::r#impl::enums::wall_connection_type::WallConnectionType;
use crate::block::property::r#impl::enums::wood_type::WoodType;
use crate::block::property::r#type::block_property_type::BlockProperty;
use crate::math::enums::axis::Axis;
use crate::math::enums::block_face::BlockFace;
use once_cell::sync::Lazy;
use strum::VariantNames;

pub const ACTIVE: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_boolean(
        "active", 
        false
    )
);

pub const AGE_16: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_int(
        "age", 
        0, 
        15, 
        0
    )
);

pub const AGE_6: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_int(
        "age", 
        0, 
        5, 
        0
    )
);

pub const AGE_4: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_int(
        "age", 
        0, 
        3, 
        0
    )
);

pub const AGE_3: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_int(
        "age", 
        0, 
        2, 
        0
    )
);

pub const AGE_BIT: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_boolean(
        "age_bit", 
        false
    )
);

pub const ALLOW_UNDERWATER_BIT: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_boolean(
        "allow_underwater_bit", 
        false
    )
);

pub const ATTACHED_BIT: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_boolean(
        "attached_bit", 
        false
    )
);

pub const ATTACHMENT: Lazy<BlockProperty> = Lazy::new(|| 
    BlockProperty::create_enum(
        "attachment", 
        Attachment::VARIANTS,
        Attachment::VARIANTS[0]
    )
);

pub const BAMBOO_LEAF_SIZE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "bamboo_leaf_size",
        BambooLeafSize::VARIANTS,
        BambooLeafSize::VARIANTS[0]
    )
);

pub const BAMBOO_STALK_THICKNESS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "bamboo_stalk_thickness",
        BambooStalkThickness::VARIANTS,
        BambooStalkThickness::VARIANTS[0]
    )
);

pub const BIG_DRIPLEAF_HEAD: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "big_dripleaf_head",
        false
    )
);

pub const BIG_DRIPLEAF_TILT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "big_dripleaf_tilt",
        BigDripleafTilt::VARIANTS,
        BigDripleafTilt::VARIANTS[0]
    )
);

pub const BITE_COUNTER: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "bite_counter",
        0,
        6,
        0
    )
);

pub const BLOCK_LIGHT_LEVEL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "block_light_level",
        0,
        15,
        0
    )
);

pub const BLOOM: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "bloom",
        false
    )
);

pub const BOOKS_STORED: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "books_stored",
        0,
        63,
        0,
    )
);

pub const BREWING_STAND_SLOT_A_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "brewing_stand_slot_a_bit",
        false
    )
);

pub const BREWING_STAND_SLOT_B_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "brewing_stand_slot_b_bit",
        false
    )
);

pub const BREWING_STAND_SLOT_C_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "brewing_stand_slot_c_bit",
        false
    )
);

pub const BRUSHED_PROGRESS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "brushed_progress",
        0,
        3,
        0
    )
);

pub const BUTTON_PRESSED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "button_pressed_bit",
        false
    )
);

pub const CAN_SUMMON: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "can_summon",
        false
    )
);

pub const CANDLES: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "candles",
        0,
        3,
        0
    )
);

pub const CAULDRON_LIQUID: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "cauldron_liquid",
        CauldronLiquid::VARIANTS,
        CauldronLiquid::VARIANTS[0]
    )
);

pub const CHEMISTRY_TABLE_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "chemistry_table_type",
        ChemistryTableType::VARIANTS,
        ChemistryTableType::VARIANTS[0]
    )
);

pub const CHISEL_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "chisel_type",
        ChiselType::VARIANTS,
        ChiselType::VARIANTS[0]
    )
);

pub const CLUSTER_COUNT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "cluster_count",
        0,
        3,
        0
    )
);

pub const COLOR: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "color",
        Color::VARIANTS,
        Color::VARIANTS[0]
    )
);

pub const COLOR_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "color_bit",
        false
    )
);

pub const COMPOSTER_FILL_LEVEL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "composter_fill_level",
        0,
        8,
        0
    )
);

pub const CONDITIONAL_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "conditional_bit",
        false
    )
);

pub const CORAL_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "coral_direction",
        0,
        3,
        0
    )
);

pub const CORAL_FAN_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "coral_fan_direction",
        0,
        1,
        0
    )
);

pub const CORAL_HANG_TYPE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "coral_hang_type_bit",
        false
    )
);

pub const COVERED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "covered_bit",
        false
    )
);

pub const CRACKED_STATE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "cracked_state",
        CrackedState::VARIANTS,
        CrackedState::VARIANTS[0]
    )
);

pub const CRAFTING: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "crafting",
        false
    )
);

pub const CREAKING_HEART_STATE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "creaking_heart_state",
        CreakingHeartState::VARIANTS,
        CreakingHeartState::VARIANTS[0]
    )
);

pub const DAMAGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "damage",
        Damage::VARIANTS,
        Damage::VARIANTS[0]
    )
);

pub const DEAD_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "dead_bit",
        false
    )
);

pub const DEPRECATED: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "deprecated",
        0,
        3,
        0
    )
);

pub const DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "direction",
        0,
        3,
        0
    )
);

pub const DIRT_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "dirt_type",
        DirtType::VARIANTS,
        DirtType::VARIANTS[0]
    )
);

pub const DISARMED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "disarmed_bit",
        false
    )
);

pub const DOOR_HINGE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "door_hinge_bit",
        false
    )
);

pub const DOUBLE_PLANT_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "double_plant_type",
        DoublePlantType::VARIANTS,
        DoublePlantType::VARIANTS[0]
    )
);

pub const DRAG_DOWN: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "drag_down",
        false
    )
);

pub const DRIPSTONE_THICKNESS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "dripstone_thickness",
        DripstoneThickness::VARIANTS,
        DripstoneThickness::VARIANTS[0]
    )
);

pub const END_PORTAL_EYE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "end_portal_eye_bit",
        false
    )
);

pub const EXPLODE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "explode_bit",
        false
    )
);

pub const EXTINGUISHED: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "extinguished",
        false,
    )
);

pub const CORAL_COLOR: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "coral_color",
        CoralColor::VARIANTS,
        CoralColor::VARIANTS[0]
    )
);

pub const FACING_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "facing_direction",
        0,
        5,
        0
    )
);

pub const FILL_LEVEL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "fill_level",
        0,
        6,
        0
    )
);

pub const GROUND_SIGN_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "ground_sign_direction",
        0,
        15,
        0
    )
);

pub const GROWING_PLANT_AGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "growing_plant_age",
        0,
        25,
        0
    )
);

pub const GROWTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "growth",
        0,
        7,
        0
    )
);

pub const HANGING: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "hanging",
        false
    )
);

pub const HEAD_PIECE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "head_piece_bit",
        false
    )
);

pub const HEIGHT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "height",
        0,
        7,
        0,
        
    )
);

pub const HONEY_LEVEL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "honey_level",
        0,
        5,
        0
    )
);

pub const HUGE_MUSHROOM_BITS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "huge_mushroom_bits",
        0,
        15,
        0
    )
);

pub const IN_WALL_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "in_wall_bit",
        false
    )
);

pub const INFINIBURN_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "infiniburn_bit",
        false
    )
);

pub const ITEM_FRAME_MAP_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "item_frame_map_bit",
        false
    )
);

pub const ITEM_FRAME_PHOTO_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "item_frame_photo_bit",
        false
    )
);

pub const KELP_AGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "kelp_age",
        0,
        25,
        0
    )
);

pub const LEVER_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "lever_direction",
        LeverDirection::VARIANTS,
        LeverDirection::VARIANTS[0]
    )
);

pub const LIQUID_DEPTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "liquid_depth",
        0,
        15,
        0
    )
);

pub const LIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "lit",
        false
    )
);

pub const MINECRAFT_BLOCK_FACE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "minecraft:block_face",
        BlockFace::VARIANTS,
        BlockFace::VARIANTS[0]
    )
);

pub const MINECRAFT_CARDINAL_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "minecraft:cardinal_direction",
        MinecraftCardinalDirection::VARIANTS,
        MinecraftCardinalDirection::VARIANTS[0]
    )
);

pub const MINECRAFT_FACING_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "minecraft:facing_direction",
        BlockFace::VARIANTS,
        BlockFace::VARIANTS[0]
    )
);

pub const MINECRAFT_VERTICAL_HALF: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "minecraft:vertical_half",
        MinecraftVerticalHalf::VARIANTS,
        MinecraftVerticalHalf::VARIANTS[0]
    )
);

pub const MOISTURIZED_AMOUNT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "moisturized_amount",
        0,
        7,
        0
    )
);

pub const MONSTER_EGG_STONE_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "monster_egg_stone_type",
        MonsterEggStoneType::VARIANTS,
        MonsterEggStoneType::VARIANTS[0]
    )
);

pub const MULTI_FACE_DIRECTION_BITS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "multi_face_direction_bits",
        0, 
        63,
        0
    )
);

pub const NEW_LEAF_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "new_leaf_type",
        NewLeafType::VARIANTS,
        NewLeafType::VARIANTS[0]
    )
);

pub const OCCUPIED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "occupied_bit",
        false
    )
);

pub const OLD_LEAF_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "old_leaf_type",
        OldLeafType::VARIANTS,
        OldLeafType::VARIANTS[0]
    )
);

pub const OPEN_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "open_bit",
        false
    )
);

pub const ORIENTATION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "orientation",
        Orientation::VARIANTS,
        Orientation::VARIANTS[0]
    )
);

pub const OUTPUT_LIT_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "output_lit_bit",
        false
    )
);

pub const OUTPUT_SUBTRACT_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "output_subtract_bit",
        false
    )
);

pub const PERSISTENT_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "persistent_bit",
        false
    )
);

pub const PILLAR_AXIS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "pillar_axis",
        Axis::VARIANTS,
        Axis::VARIANTS[0]
    )
);

pub const PORTAL_AXIS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "portal_axis",
        PortalAxis::VARIANTS,
        PortalAxis::VARIANTS[0]
    )
);

pub const POWERED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "powered_bit",
        false
    )
);

pub const PRISMARINE_BLOCK_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "prismarine_block_type",
        PrismarineBlockType::VARIANTS,
        PrismarineBlockType::VARIANTS[0]
    )
);

pub const PROPAGULE_STAGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "propagule_stage",
        0,
        4,
        0
    )
);

pub const RAIL_DATA_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "rail_data_bit",
        false
    )
);

pub const RAIL_DIRECTION_10: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "rail_direction_10",
        0,
        9,
        0
    )
);

pub const RAIL_DIRECTION_6: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "rail_direction_6",
        0,
        5,
        0
    )
);

pub const REDSTONE_SIGNAL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "redstone_signal",
        0,
        15,
        0
    )
);

pub const REPEATER_DELAY: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "repeater_delay",
        0,
        3,
        0
    )
);

pub const RESPAWN_ANCHOR_CHARGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "respawn_anchor_charge",
        0,
        4,
        0
    )
);

pub const ROTATION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "rotation",
        0,
        3,
        0
    )
);

pub const SAND_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "sand_type",
        SandType::VARIANTS,
        SandType::VARIANTS[0]
    )
);

pub const SCULK_SENSOR_PHASE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "sculk_sensor_phase",
        0,
        2,
        0
    )
);

pub const SEA_GRASS_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "sea_grass_type",
        SeaGrassType::VARIANTS,
        SeaGrassType::VARIANTS[0]
    )
);

pub const SPONGE_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "sponge_type",
        SpongeType::VARIANTS,
        SpongeType::VARIANTS[0]
    )
);

pub const STABILITY: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "stability",
        0,
        7,
        0
    )
);

pub const STABILITY_CHECK: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "stability_check",
        false
    )
);

pub const STONE_BRICK_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "stone_brick_type",
        StoneBrickType::VARIANTS,
        StoneBrickType::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "stone_slab_type",
        StoneSlabType::VARIANTS,
        StoneSlabType::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_2: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "stone_slab_type_2",
        StoneSlabType2::VARIANTS,
        StoneSlabType2::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_3: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "stone_slab_type_3",
        StoneSlabType3::VARIANTS,
        StoneSlabType3::VARIANTS[0]
    )
);

pub const STONE_SLAB_TYPE_4: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "stone_slab_type_4",
        StoneSlabType4::VARIANTS,
        StoneSlabType4::VARIANTS[0]
    )
);

pub const STRIPPED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "stripped_bit",
        false
    )
);

pub const STRUCTURE_BLOCK_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "structure_block_type",
        StructureBlockType::VARIANTS,
        StructureBlockType::VARIANTS[0]
    )
);

pub const STRUCTURE_VOID_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "structure_void_type",
        StructureVoidType::VARIANTS,
        StructureVoidType::VARIANTS[0]
    )
);

pub const SUSPENDED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "suspended_bit",
        false
    )
);

pub const TALL_GRASS_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "tall_grass_type",
        TallGrassType::VARIANTS,
        TallGrassType::VARIANTS[0]
    )
);

pub const TOGGLE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "toggle_bit",
        false
    )
);

pub const TORCH_FACING_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "torch_facing_direction",
        TorchFacingDirection::VARIANTS,
        TorchFacingDirection::VARIANTS[0]
    )
);

pub const TRIGGERED_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "triggered_bit",
        false
    )
);

pub const TURTLE_EGG_COUNT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "turtle_egg_count",
        TurtleEggCount::VARIANTS,
        TurtleEggCount::VARIANTS[0]
    )
);

pub const TWISTING_VINES_AGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "twisting_vines_age",
        0,
        25,
        0
    )
);

pub const UPDATE_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "update_bit",
        false
    )
);

pub const UPPER_BLOCK_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "upper_block_bit",
        false
    )
);

pub const UPSIDE_DOWN_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "upside_down_bit",
        false
    )
);

pub const VINE_DIRECTION_BITS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "vine_direction_bits",
        0,
        15,
        0
    )
);

pub const WALL_BLOCK_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wall_block_type",
        WallBlockType::VARIANTS,
        WallBlockType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_EAST: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wall_connection_type_east",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_NORTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wall_connection_type_north",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_SOUTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wall_connection_type_south",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const WALL_CONNECTION_TYPE_WEST: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wall_connection_type_west",
        WallConnectionType::VARIANTS,
        WallConnectionType::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_EAST: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "pale_moss_carpet_side_east",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_NORTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "pale_moss_carpet_side_north",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_SOUTH: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "pale_moss_carpet_side_south",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const PALE_MOSS_CARPET_SIDE_WEST: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "pale_moss_carpet_side_west",
        PaleMossCarpetSide::VARIANTS,
        PaleMossCarpetSide::VARIANTS[0]
    )
);

pub const TIP: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "tip",
        false
    )
);

pub const NATURAL: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "natural",
        false
    )
);

pub const WALL_POST_BIT: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "wall_post_bit",
        false
    )
);

pub const WEEPING_VINES_AGE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "weeping_vines_age",
        0,
        25,
        0,
    )
);

pub const WEIRDO_DIRECTION: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "weirdo_direction",
        0,
        3,
        0
    )
);

pub const WOOD_TYPE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "wood_type",
        WoodType::VARIANTS,
        WoodType::VARIANTS[0]
    )
);

pub const TRIAL_SPAWNER_STATE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_int(
        "trial_spawner_state",
        0,
        5,
        0
    )
);

pub const VAULT_STATE: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_enum(
        "vault_state",
        VaultState::VARIANTS,
        VaultState::VARIANTS[0]
    )
);

pub const OMINOUS: Lazy<BlockProperty> = Lazy::new(||
    BlockProperty::create_boolean(
        "ominous",
        false
    )
);