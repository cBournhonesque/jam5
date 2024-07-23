use bevy::prelude::{Component, Entity, Reflect};
use lightyear::prelude::{Deserialize, Serialize};

/// Message sent to the client to notify that we are spawning an entity
#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SpawnPlayerMessage {
    bike: Entity,
    trail: Entity,
    zones: Entity,
}