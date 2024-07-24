use bevy::ecs::entity::MapEntities;
use bevy::prelude::{Component, Entity, EntityMapper, Reflect};
use lightyear::prelude::{Deserialize, Serialize};

/// Message sent to the client to notify that we are spawning an entity
#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SpawnPlayerMessage {
    bike: Entity,
    trail: Entity,
    zones: Entity,
}

#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct KilledByMessage {
    pub killer: Entity,
}

impl MapEntities for KilledByMessage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.killer = entity_mapper.map_entity(self.killer);
    }
}

#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct KillMessage {
    pub killed: Entity,
}

impl MapEntities for KillMessage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.killed = entity_mapper.map_entity(self.killed);
    }
}
