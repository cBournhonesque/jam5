use avian2d::position::Position;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::hashbrown::HashMap;
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
    mut q_bikes: Query<(&Position, &mut Zones), With<BikeMarker>>,
    mut trails: Query<(&Parent, &mut Trail)>
) {
    for (parent, mut trail) in trails.iter_mut() {
        if let Ok((position, mut zones)) = q_bikes.get_mut(parent.get()) {
            if let Some(shape) = trail.try_add_point(position.0) {
                trail.line.clear();
                zones.add_zone(Zone::new(shape));
            }
        }
    }

    // // cut out all other zones
    // for (bike, _, _, mut zones) in bikes.iter_mut() {
    //     for (client_id, zone) in cut_zones.iter() {
    //         // we don't cut our own zones
    //         if bike.client_id != *client_id {
    //             zones.cut_out_zones(zone);
    //         }
    //     }
    // }
}
