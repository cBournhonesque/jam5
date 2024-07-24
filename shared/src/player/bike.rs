use super::trail::Trail;
use crate::player::scores::{Score, Stats};
use crate::player::zone::Zones;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use lightyear::prelude::*;

pub const BASE_SPEED: f32 = 200.0;
pub const FAST_SPEED: f32 = 600.0;
pub const DRAG: f32 = 10.0;
pub const ACCEL: f32 = 300.0;
pub const FAST_SPEED_MAX_SPEED_DISTANCE: f32 = 500.0; // we lerp from BASE_SPEED to FAST_SPEED based on this mouse distance
pub const MAX_ROTATION_SPEED: f32 = 6.0;
pub const FAST_DRAG: f32 = 2.0;

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct ColorComponent(pub Color);

#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BikeMarker {
    pub client_id: ClientId,
    pub name: String,
    // for testing
    pub stopped: bool,
    // // TODO: these are unused right now!
    // // The trail entity associated with the bike
    // pub trail: Entity,
    // // The zones entity associated with the bike
    // pub zones: Entity,
}

// impl MapEntities for BikeMarker {
//     fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
//         self.trail = entity_mapper.map_entity(self.trail);
//         self.zones = entity_mapper.map_entity(self.zones);
//     }
// }

impl Default for BikeMarker {
    fn default() -> Self {
        Self {
            client_id: ClientId::Netcode(0),
            name: "Player".to_owned(),
            stopped: false,
            // trail: Entity::PLACEHOLDER,
            // zones: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub color: ColorComponent,
    pub score: Score,
    pub stats: Stats,
    pub name: Name,
}

impl BikeBundle {
    pub fn new_at(client_id: ClientId, position: Vec2, color: Color) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            marker: BikeMarker {
                client_id,
                ..default()
            },
            position: Position(position),
            color: ColorComponent(color),
            linear_velocity: LinearVelocity(Vector::new(0.0, 0.0)),
            name: Name::from("Bike"),
            ..default()
        }
    }
}

pub struct BikePlugin;

impl Plugin for BikePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BikeMarker>();
    }
}
