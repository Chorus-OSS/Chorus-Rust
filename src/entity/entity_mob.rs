use std::collections::HashMap;
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct EntityMob {
    // NBT fields
    active_effects: Option<Vec<HashMap<String, nbtx::Value>>>,
    air: i16,
    armor: [HashMap<String, nbtx::Value>; 5],
    attack_time: i16,
    attributes: Vec<HashMap<String, nbtx::Value>>,
    body_rot: Option<f32>,
    bound_x: i32,
    bound_y: i32,
    bound_z: i32,
    can_pickup_items: bool,
    dead: bool,
    death_time: i16,
    has_bound_origin: bool,
    has_set_can_pickup_items: bool,
    hurt_time: i16,
    leasher_id: i64,
    limited_life: i64,
    mainhand: [HashMap<String, nbtx::Value>; 1],
    natural_spawn: bool,
    offhand: [HashMap<String, nbtx::Value>; 1],
    persisting_offers: Option<HashMap<String, nbtx::Value>>,
    persisting_riches: Option<i32>,
    surface: bool,
    target_captain_id: Option<i64>,
    target_id: i64,
    trade_experience: Option<i32>,
    trade_tier: Option<i32>,
    wants_to_be_jockey: Option<bool>,
}