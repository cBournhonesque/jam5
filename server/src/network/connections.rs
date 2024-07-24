//! Handle client connections
use avian2d::prelude::RigidBody;
use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use shared::player::bike::{BikeBundle, BikeMarker};
use shared::player::trail::{Trail, TrailBundle};
use shared::player::zone::{Zones, ZonesBundle};

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
    let bike = commands
        .spawn((
            BikeBundle::new_at(client_id, pos, color),
            RigidBody::Kinematic,
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
        ))
        .id();
    let trail = commands
        .spawn((
            TrailBundle::new_at(pos),
            // To replicate the parent/child hierarchy
            ParentSync::default(),
            // Enable delta compression when replicating the trail
            DeltaCompression::<Trail>::default(),
            Replicate {
                // TODO: add network relevance
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    let zones = commands
        .spawn((
            ZonesBundle {
                zones: Zones {
                    owner_client_id: client_id,
                    ..default()
                },
                ..default()
            },
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
        ))
        .id();
    commands
        .entity(bike)
        // .insert(BikeMarker {
        //     client_id,
        //     stopped: false,
        //     ..default()
        //     // trail,
        //     // zones,
        // })
        .add_child(trail)
        .add_child(zones);
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
