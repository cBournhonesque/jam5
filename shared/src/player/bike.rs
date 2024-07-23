use super::trail::Trail;
use crate::player::zone::Zones;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::connection::netcode::ClientId;
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

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct BikeMarker {
    pub client_id: ClientId,
    pub stopped: bool,
}

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub color: ColorComponent,
    pub trail: Trail,
    pub zones: Zones,
    pub name: Name,
}

impl BikeBundle {
    pub fn new_at(client_id: ClientId, position: Vec2, color: Color) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            marker: BikeMarker {
                client_id,
                stopped: false,
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
    fn build(&self, app: &mut App) {}
}
