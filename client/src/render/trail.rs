//! How to draw trails

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::trail::Trail;
use shared::player::zone::{Zone, Zones};

pub struct TrailRenderPlugin;

#[derive(Component)]
pub struct TrailRenderMarker;

impl Plugin for TrailRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_trail_path.after(MainSet::Receive));
    }
}

/// Update the path of a zone when the zone gets updated
fn update_trail_path(
    trail_query: Query<&Trail, Changed<Trail>>,
    mut trail_render_query: Query<(&Parent, &mut Path), With<TrailRenderMarker>>,
) {
    for (parent, mut path) in trail_render_query.iter_mut() {
        if let Ok(trail) = trail_query.get(parent.get()) {
            *path = trail.into();
        }
    }
}