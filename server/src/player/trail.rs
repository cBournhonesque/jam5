use avian2d::position::Position;
use bevy::prelude::*;
use lightyear::{
    prelude::{server::Replicate, server::SyncTarget},
    shared::replication::network_target::NetworkTarget,
};
use shared::player::{bike::BikeMarker, trail::Trail, zone::Zone};
pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, mark_trail_system);
    }
}

fn mark_trail_system(
    mut commands: Commands,
    mut query: Query<(&Position, &mut Trail), With<BikeMarker>>,
) {
    for (position, mut trail) in query.iter_mut() {
        let point = position.0;
        if let Some(shape) = trail.try_add_point(point) {
            commands.spawn((Zone::new(shape), Replicate::default()));
            // TODO: spawn the shape https://docs.rs/parry2d/latest/parry2d/shape/struct.SharedShape.html#method.round_convex_decomposition_with_params
            // TODO: temporarily disable the trail?
            // TODO: total up surface area and increment score based on that?
        }
    }
}
