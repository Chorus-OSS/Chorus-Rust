use shipyard::Component;

#[derive(Component)]
pub struct EntityVillager {
    // NBT fields
    willing: bool,
}
