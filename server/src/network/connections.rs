//! Handle client connections

use avian2d::prelude::{Position, RigidBody};
use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use shared::player::bike::{BikeBundle, BikeMarker};
use shared::player::trail::{Trail, TrailBundle};
use shared::player::zone::{Zones, ZonesBundle};
use std::time::Duration;

// We need to make sure that the bike is replicated before the trail/zone
// (we cannot send them as a single replication group because of DeltaCompression)
// We will spawn the trail and zone after a delay
const TRAIL_SPAWN_DELAY: Duration = Duration::from_millis(800);

#[derive(Component)]
pub struct DelaySpawnTrailZone {
    timer: Timer,
}

/// Spawn the trail/zones after a small delay so that the ParentSync components
/// can map entities correctly
pub(crate) fn spawn_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &BikeMarker, &Position, &mut DelaySpawnTrailZone)>,
) {
    for (entity, bike_marker, pos, mut delay) in query.iter_mut() {
        let client_id = bike_marker.client_id;
        delay.timer.tick(time.delta());
        if delay.timer.finished() {
            // remove the timer
            commands
                .entity(entity)
                .remove::<DelaySpawnTrailZone>()
                // add the trail and zones
                .with_children(|parent| {
                    parent.spawn((
                        TrailBundle::new_at(pos.0),
                        // To replicate the parent/child hierarchy
                        ParentSync::default(),
                        // Enable delta compression when replicating the trail
                        DeltaCompression::<Trail>::default(),
                        Replicate {
                            // TODO: add network relevance
                            controlled_by: ControlledBy {
                                target: NetworkTarget::Single(bike_marker.client_id),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    parent.spawn((
                        ZonesBundle::default(),
                        // To replicate the parent/child hierarchy
                        ParentSync::default(),
                        // Enable delta compression when replicating the zones
                        DeltaCompression::<Zones>::default(),
                        Replicate {
                            // TODO: add network relevance
                            controlled_by: ControlledBy {
                                target: NetworkTarget::Single(client_id),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        }
    }
}

/// Spawn a new bike when a player connects, along with a `Trail` and a `Zones` entities
pub(crate) fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    info!("Spawning bike for client {}", trigger.event().client_id);
    let client_id = trigger.event().client_id;
    let color = color_from_client_id(client_id.to_bits());
    let pos = Vec2::new(0.0, 0.0);

    // TODO: THIS ONLY WORKS IF BIKE ENTITY IS REPLICATED BEFORE TRAIL/ZONE!!
    //  THIS MIGHT NOT BE THE CASE IF THEY ARRIVE IN DIFFERENT PACKETS!
    //  WHAT SHOULD WE DO? SEND MESSAGES?

    // NOTE: for complicated reasons related to lightyear:
    //  - each entity must be replicated in a different replication group (so that delta compression works)
    //  - but the trail/zones must be replicated before the bike, so that the BikeMarker has a pointer to the correct entities
    //  - but the trail/zones must be replicated after the bike, so that the ParentSync has a pointer to the correct entities
    commands.spawn((
        BikeBundle::new_at(client_id, pos, color),
        RigidBody::Kinematic,
        DelaySpawnTrailZone {
            timer: Timer::new(TRAIL_SPAWN_DELAY, TimerMode::Once),
        },
        Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            // TODO: add network relevance
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            // do not automatically replicate the hierarchy, because we want the trail and the bike
            // to be part of different ReplicationGroups. This is for delta-compression to work correctly.
            hierarchy: ReplicateHierarchy { recursive: false },
            ..default()
        },
    ));
}

pub fn color_from_client_id(client_id: u64) -> Color {
    let available_colors = [
        css::LIMEGREEN,
        css::PINK,
        css::YELLOW,
        css::AQUA,
        css::CRIMSON,
        css::GOLD,
        css::ORANGE_RED,
        css::SILVER,
        css::SALMON,
        css::YELLOW_GREEN,
        css::WHITE,
        css::RED,
    ];
    let color = available_colors
        .get(client_id as usize % available_colors.len())
        .unwrap();
    return Color::Srgba(*color);
}
