use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

use super::trail::Trail;

pub const BASE_SPEED: f32 = 200.0;
pub const FAST_SPEED: f32 = 600.0;
pub const DRAG: f32 = 10.0;
pub const ACCEL: f32 = 300.0;
pub const FAST_SPEED_MAX_SPEED_DISTANCE: f32 = 500.0; // we lerp from BASE_SPEED to FAST_SPEED based on this mouse distance
pub const MAX_ROTATION_SPEED: f32 = 5.0;


#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct ColorComponent(pub Color);

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct BikeMarker;

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub color: ColorComponent,
    pub name: Name,
    pub collider: Collider,
    // pub trail: Trail,
}

impl BikeBundle {
    pub fn new_at(position: Vector, color: Color) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            position: Position(position),
            color: ColorComponent(color),
            // non-zero velocity to avoid self-collisions
            linear_velocity: LinearVelocity(Vector::new(0.1, 0.1)),
            name: Name::from("Bike"),
            collider: Collider::circle(0.0),
            ..default()
        }
    }
}

pub struct BikePlugin;

impl Plugin for BikePlugin {
    fn build(&self, app: &mut App) {}
}
