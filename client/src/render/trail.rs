//! How to draw trails

use bevy::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::bike::ColorComponent;
use shared::player::trail::Trail;

pub struct TrailRenderPlugin;

#[derive(Component)]
pub struct TrailRenderMarker;

impl Plugin for TrailRenderPlugin {
    fn build(&self, app: &mut App) {
        // update the trail path after Receive, but before rendering
        app.add_systems(Update, update_trail_path);
    }
}

/// Update the path of a zone when the zone gets updated
fn update_trail_path(
    mut trail_query: Query<(&Trail, &mut Path), (Changed<Trail>, With<TrailRenderMarker>)>,
) {
    for (trail, mut path) in trail_query.iter_mut() {
        // info!(?trail);
        *path = trail.into();
    }