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
pub struct BikeMarker;

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub trail: Trail,
}

impl BikeBundle {
    pub fn new_at(position: Vec2) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            position: Position(position),
            linear_velocity: LinearVelocity(Vector::new(0.0, 0.0)),
            ..default()
        }
    }
}

pub struct BikePlugin;

impl Plugin for BikePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, mark_trail_system);
    }
}

fn mark_trail_system(mut query: Query<(&Position, &mut Trail), With<BikeMarker>>) {
    println!("marking trail");
    for (position, mut trail) in query.iter_mut() {
        println!("marking trail at {:?}", position.0);
        let point = position.0;
        trail.add_point(point);
    }
}
