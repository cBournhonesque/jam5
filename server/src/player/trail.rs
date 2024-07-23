use avian2d::position::Position;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use lightyear::connection::netcode::ClientId;
use lightyear::prelude::client::Predicted;
use shared::physics::FixedSet;
use shared::player::trail::ADD_POINT_INTERVAL;
use shared::player::zone::Zones;
use shared::player::{bike::BikeMarker, trail::Trail, zone::Zone};

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            // after we have advanced objects with physics, maybe add a point
            mark_trail_system
                .run_if(on_timer(ADD_POINT_INTERVAL))
                .chain()
                .after(FixedSet::Physics),
        );
        // app.add_systems(FixedUpdate, mark_trail_system);
    }
}

/// Add a new point to the trail and update the zones accordingly
fn mark_trail_system(
    mut bikes: Query<(&Position, &BikeMarker)>,
    mut trails: Query<(&Parent, &mut Trail)>,
    mut zones_query: Query<&mut Zones>,
) {
    let mut cut_zones = HashMap::<ClientId, Zone>::new();
    for (parent, mut trail) in trails.iter_mut() {
        if let Ok((position, bike_marker)) = bikes.get_mut(parent.get()) {
            if let Some(shape) = trail.try_add_point(position.0) {
                trail.line.clear();
                if let Ok(mut zones) = zones_query.get_mut(bike_marker.zones) {
                    let new_zone = Zone::new(shape);
                    zones.add_zone(new_zone.clone());
                    cut_zones.insert(bike_marker.client_id, new_zone);
                }
            }
        }
    }

    // cut out all other zones
    for (_, bike) in bikes.iter() {
        if let Ok(mut zones) = zones_query.get_mut(bike.zones) {
            for (client_id, zone) in cut_zones.iter() {
                // we don't cut our own zones
                if bike.client_id != *client_id {
                    zones.cut_out_zones(zone);
                }
            }
        }
    }
}
