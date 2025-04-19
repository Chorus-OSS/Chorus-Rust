use std::collections::HashMap;
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct EntityHumanoidMonster {
    // NBT fields
    item_in_hand: Option<HashMap<String, nbtx::Value>>
}