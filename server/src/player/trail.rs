use avian2d::position::Position;
use bevy::prelude::*;
use shared::player::{
    bike::{BikeMarker, ColorComponent},
    trail::Trail,
    zone::{Zone, ZoneManager},
};
pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, mark_trail_system);
    }
}

fn mark_trail_system(
    mut zone_manager: ResMut<ZoneManager>,
    mut q_bikes: Query<(&BikeMarker, &Position, &mut Trail, &ColorComponent)>,
) {
    for (bike, position, mut trail, color) in q_bikes.iter_mut() {
        let point = position.0;
        if let Some(shape) = trail.try_add_point(point) {
            trail.line.clear();

            let new_zone = Zone::new(shape, Color::WHITE);
            zone_manager.add_zone(bike.client_id, new_zone.clone());
        }
    }
}
