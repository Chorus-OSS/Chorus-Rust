use std::collections::HashMap;
use shipyard::Component;

#[derive(Component)]
pub struct EntityMinecart {
    // NBT fields
    custom_display_tile: Option<bool>,
    display_block: Option<HashMap<String, nbtx::Value>>,
    display_offset: Option<i32>,
}