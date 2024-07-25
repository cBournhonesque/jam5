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
pub const OUR_ZONE_SPEED_MULTIPLIER: f32 = 1.5;
pub const ENEMY_ZONE_SPEED_MULTIPLIER: f32 = 0.8;

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct ColorComponent(pub Color);

impl ColorComponent {
    pub fn overbright(&self, amount: f32) -> Color {
        (self.0.to_linear() * amount).into()
    }
}

#[derive(Deref, Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ClientIdMarker(pub ClientId);

impl Default for ClientIdMarker {
    fn default() -> Self {
        Self(ClientId::Netcode(0))
    }
}

#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BikeMarker {
    pub name: String,
    #[cfg(feature = "dev")]
    pub paused: bool,
}

impl BikeMarker {
    pub fn new(name: String) -> Self {
        Self {
            name,
            #[cfg(feature = "dev")]
            paused: false,
        }
    }
}

impl Default for BikeMarker {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            #[cfg(feature = "dev")]
            paused: false,
        }
    }
}

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub client_id: ClientIdMarker,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub color: ColorComponent,
    pub score: Score,
    pub stats: Stats,
    pub name: Name,
}

impl BikeBundle {
    pub fn new_at(client_id: ClientId, name: String, position: Vec2, color: Color) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            marker: BikeMarker::new(name),
            client_id: ClientIdMarker(client_id),
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
        app.register_type::<ClientIdMarker>();
    }
}
