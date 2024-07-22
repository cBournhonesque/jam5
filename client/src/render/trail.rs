//! How to draw trails

use bevy::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::trail::Trail;
use shared::player::zone::{Zone, Zones};

pub struct TrailRenderPlugin;

#[derive(Component)]
pub struct TrailRenderMarker;

impl Plugin for TrailRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_trail_path.after(MainSet::Receive));

        // // Draw after TransformPropagate and VisualInterpolation
        // app.add_systems(
        //     PostUpdate,
        //     (
        //         draw_trail,
        //     )
        //         .after(TransformPropagate),
        // );
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

fn draw_trail(mut gizmos: Gizmos, query: Query<(&Trail, &ColorComponent), With<BikeMarker>>) {
    for (trail, color) in query.iter() {
        let trail_color = Color::Hsva(Hsva {
            saturation: 0.4,
            ..Hsva::from(color.0)
        });
        if trail.line.len() < 2 {
            continue;
        }
        for i in 0..trail.line.len() - 1 {
            let start = trail.line[i];
            let end = trail.line[i + 1];
            gizmos.line_2d(start, end, trail_color);
        }
    }
}