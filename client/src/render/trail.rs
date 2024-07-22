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

        // Draw after TransformPropagate and VisualInterpolation
        app.add_systems(
            PostUpdate,
            (
                draw_trail,
            )
                .after(TransformPropagate),
        );
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
}

fn draw_trail(mut gizmos: Gizmos,
              bike_query: Query<&ColorComponent>,
              query: Query<(&Parent, &Trail)>) {
    for (parent, trail) in query.iter() {
        if let Ok(color) = bike_query.get(parent.get()) {
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
}