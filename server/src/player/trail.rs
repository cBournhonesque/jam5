use avian2d::position::Position;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use shared::physics::FixedSet;
use shared::player::{
    bike::{BikeMarker},
    trail::Trail,
    zone::{Zone},
};
use shared::player::trail::ADD_POINT_INTERVAL;
use shared::player::zone::Zones;

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            // after we have advanced objects with physics, maybe add a point
            mark_trail_system.run_if(on_timer(ADD_POINT_INTERVAL))
                .chain()
                .after(FixedSet::Physics)
        );
        // app.add_systems(FixedUpdate, mark_trail_system);
    }
}

fn mark_trail_system(
    mut q_bikes: Query<(&Position, &mut Trail, &mut Zones), With<BikeMarker>>,
) {
    for (position, mut trail, mut zones) in q_bikes.iter_mut() {
        if let Some(shape) = trail.try_add_point(position.0) {
            trail.line.clear();
            zones.add_zone(Zone::new(shape));
        }
    }
}