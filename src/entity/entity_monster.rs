use shipyard::Component;

#[derive(Component)]
pub struct EntityMonster {
    // NBT fields
    spawned_by_night: bool,
}