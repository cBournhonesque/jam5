use crate::player::scores::Stats;
use bevy::color::Color;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::{Component, Entity, EntityMapper, Reflect, Vec2};
use lightyear::prelude::{Deserialize, Serialize};

/// Message sent from the client to spawn the player with a given name
#[derive(Reflect, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SpawnPlayerMessage {
    pub name: String,
}

#[derive(Reflect, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BikeDeathMessage {
    pub color: Color,
    pub position: Vec2,
}

#[derive(Reflect, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct KilledByMessage {
    pub killer: Entity,
    pub stats: Stats,
}

impl MapEntities for KilledByMessage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.killer = entity_mapper.map_entity(self.killer);
    }
}

#[derive(Reflect, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct KillMessage {
    pub killed: Entity,
}

impl MapEntities for KillMessage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.killed = entity_mapper.map_entity(self.killed);
    }
}
