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

fn mark_trail_system(mut bikes: Query<(&BikeMarker, &Position, &mut Trail, &mut Zones)>) {
    let mut cut_zones = HashMap::<ClientId, Zone>::new();

    for (bike, position, mut trail, mut zones) in bikes.iter_mut() {
        if let Some(shape) = trail.try_add_point(position.0) {
            trail.line.clear();
            let new_zone = Zone::new(shape);
            zones.add_zone(new_zone.clone());
            cut_zones.insert(bike.client_id, new_zone);
        }
    }

    // cut out all other zones
    for (bike, _, _, mut zones) in bikes.iter_mut() {
        for (client_id, zone) in cut_zones.iter() {
            // we don't cut our own zones
            if bike.client_id != *client_id {
                zones.cut_out_zones(zone);
            }
        }
    }
}
