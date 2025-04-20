use std::sync::Arc;
use shipyard::Component;
use crate::entity::entity::Entity;
use crate::entity::{entity_id, entity_mob};
use crate::entity::entity_mob::EntityMob;
use crate::level::level::Level;

#[derive(Component)]
pub struct Player {
    
}

impl Player {
    pub fn new(level: Arc<Level>) -> (Entity, EntityMob, Player) {
        let entity = Entity::default(entity_id::PLAYER.to_string(), rand::random::<i64>(), level);
        let entity_mob = EntityMob::default();
        let player = Player {};
        
        (entity, entity_mob, player)
    }
}