use std::collections::HashMap;
use shipyard::Component;
use vek::{Vec2, Vec3};
use crate::level::level::Level;

#[derive(Component)]
pub struct Entity {
    position: Vec3<f32>,
    rotation: Vec2<f32>,
    velocity: Vec3<f32>,

    level: Level,
    
    // NBT fields
    chested: bool,
    color: u8,
    color2: u8,
    custom_name: Option<String>,
    custom_name_visible: Option<bool>,
    definitions: Option<Vec<String>>,
    fall_distance: f32,
    fire: i16,
    identifier: String,
    internal_components: HashMap<String, HashMap<String, String>>,
    invulnerable: bool,
    is_angry: bool,
    is_autonomous: bool,
    is_baby: bool,
    is_eating: bool,
    is_gliding: bool,
    is_global: bool,
    is_illager_captain: bool,
    is_orphaned: bool,
    is_out_of_control: bool,
    is_roaring: bool,
    is_scared: bool,
    is_stunned: bool,
    is_swimming: bool,
    is_tamed: bool,
    is_trusting: bool,
    last_dimension_id: Option<i32>,
    links_tag: Option<HashMap<String, nbtx::Value>>,
    loot_dropped: bool,
    mark_variant: i32,
    // motion -> velocity
    on_ground: bool,
    owner_new: i64,
    persistent: bool,
    portal_cooldown: i32,
    // pos -> position
    // rotation -> rotation
    saddled: bool,
    sheared: bool,
    show_bottom: bool,
    sitting: bool,
    skin_id: i32,
    strength: i32,
    strength_max: i32,
    tags: Option<Vec<String>>,
    unique_id: i64,
    variant: i32,
}