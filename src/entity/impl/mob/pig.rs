use std::sync::Arc;
use shipyard::Component;
use crate::entity::components::ageable::Ageable;
use crate::entity::components::breedable::Breedable;
use crate::entity::entity::Entity;
use crate::entity::entity_id;
use crate::entity::entity_mob::EntityMob;
use crate::level::level::Level;

#[derive(Component)]
pub struct Pig;

impl Pig {
    pub fn new(level: Arc<Level>) -> (Entity, EntityMob, Pig, Ageable, Breedable) {
        let entity = Entity::default(entity_id::PIG.to_string(), rand::random::<i64>(), level);
        let entity_mob = EntityMob::default();
        let pig = Pig {};
        let ageable = Ageable::default();
        let breedable = Breedable::default();

        (entity, entity_mob, pig, ageable, breedable)
    }
}