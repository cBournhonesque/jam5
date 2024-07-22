//! How to render zones

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::zone::{Zone, Zones};

pub struct ZoneRenderPlugin;

#[derive(Component)]
pub struct ZoneRenderMarker;

impl Plugin for ZoneRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_zone_path.after(MainSet::Receive));
    }
}

/// Update the path of a zone when the zone gets updated
fn update_zone_path(
    zone_query: Query<&Zones, Changed<Zones>>,
    mut zone_render_query: Query<(&Parent, &mut Path), With<ZoneRenderMarker>>,
) {
    for (parent, mut path) in zone_render_query.iter_mut() {
        if let Ok(zones) = zone_query.get(parent.get()) {
            *path = zones.into();
        }
    }
}