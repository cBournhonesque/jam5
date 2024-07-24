use avian2d::position::Position;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use shared::physics::FixedSet;
use shared::player::trail::ADD_POINT_INTERVAL;
use shared::player::zone::Zones;
use shared::player::{trail::Trail, zone::Zone};
use shared::player::death::Dead;
use shared::player::scores::{Score, Stats};
use crate::player::death::PlayerKillEvent;

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            // after we have advanced objects with physics, maybe add a point
            (mark_trail_system, update_score)
                .chain()
                .run_if(on_timer(ADD_POINT_INTERVAL))
                .after(FixedSet::Physics),
        );
        // app.add_systems(FixedUpdate, mark_trail_system);
    }
}

/// Add a new point to the trail and update the zones accordingly
fn mark_trail_system(
    mut commands: Commands,
    mut bikes: Query<(Entity, &Position, &Children, &mut Stats), Without<Dead>>,
    mut trails: Query<(Entity, &Parent, &mut Trail), Without<Dead>>,
    mut zones_query: Query<(&Parent, &mut Zones), Without<Dead>>,
) {
    let mut new_zones = HashMap::<Entity, Zone>::new();
    for (trail_entity, parent, mut trail) in trails.iter_mut() {
        if let Ok((_, position, children, mut stats)) = bikes.get_mut(parent.get()) {
            if let Some(shape) = trail.try_add_point(position.0) {
                // update stats
                stats.max_trail_length = stats.max_trail_length.max(trail.len() as u32);

                trail.line.clear();
                // we find the zone entity by querying the children of Bike that are not Trail
                let zone_entity = children.into_iter().find(|entity| {
                    **entity != trail_entity
                }).unwrap();
                if let Ok((_, mut zones)) = zones_query.get_mut(*zone_entity) {
                    let new_zone = Zone::new(shape);
                    zones.add_zone(new_zone.clone());
                    new_zones.insert(parent.get(), new_zone);
                }
            }
        }
    }

    // cut out all other zones
    for (bike_entity, zone) in new_zones.iter() {
        for (parent, mut zones) in zones_query.iter_mut() {
            // we don't cut our own zone
            if parent.get() != *bike_entity {
                zones.cut_out_zones(zone);
            }
        }

        // check if a player was killed
        for (entity, position, _, _) in bikes.iter() {
            // you cannot kill yourself
            if *bike_entity != entity && zone.contains(position.0) {
                commands.trigger(PlayerKillEvent {
                    killer: *bike_entity,
                    killed: entity,
                })
            }
        }
    }
}

/// Update the player scores when the zones change
fn update_score(
    mut bikes: Query<(&mut Stats, &mut Score)>,
    mut zones_query: Query<(&Parent, &mut Zones), Changed<Zones>>,
) {
    for (parent, mut zones) in zones_query.iter_mut() {
        if let Ok((mut stats, mut scores)) = bikes.get_mut(parent.get()) {
            // we use the area / 1000 as score
            // TODO: add kills to score formula
            let score = (zones.area() / 1000.0) as u32;
            scores.score = score;
            stats.max_score = stats.max_score.max(score);
            stats.max_area = stats.max_area.max(score);
            stats.max_zones = stats.max_zones.max(zones.zones.len() as u32);
        }
    }
}
