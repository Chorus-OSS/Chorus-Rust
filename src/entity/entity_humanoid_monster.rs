use std::collections::HashMap;
use shipyard::Component;

#[derive(Component)]
pub struct EntityHumanoidMonster {
    // NBT fields
    item_in_hand: Option<HashMap<String, nbtx::Value>>
}