use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

pub const BIKE_VELOCITY: f32 = 10.0;


#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct BikeMarker;

#[derive(Bundle, Default)]
pub struct BikeBundle {
    pub marker: BikeMarker,
    pub position : Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    // TODO: collision? friction?
}

impl BikeBundle {
    pub fn new_at(position: Vec2) -> Self {
        // TODO: spawn at a random position on the map
        Self {
            position: Position(position),
            linear_velocity: LinearVelocity(Vector::new(0.0, BIKE_VELOCITY)),
            ..default()
        }
    }
}


pub struct BikePlugin;

impl Plugin for BikePlugin {
    fn build(&self, app: &mut App) {
    }
}



