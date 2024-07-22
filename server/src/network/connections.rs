//! Handle client connections
use avian2d::prelude::RigidBody;
use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use shared::player::bike::BikeBundle;
use shared::player::trail::{Trail, TrailBundle};

/// Spawn a new bike when a player connects
pub(crate) fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    info!("Spawning bike for client {}", trigger.event().client_id);
    let client_id = trigger.event().client_id;
    let color = color_from_client_id(client_id.to_bits());
    let pos =  Vec2::new(0.0, 0.0);
    commands.spawn((
        BikeBundle::new_at(client_id.to_bits(), pos, color),
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
            hierarchy: ReplicateHierarchy {
                recursive: false,
            },
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            TrailBundle::new_at(pos),
            // To replicate the parent/child hierarchy
            ParentSync::default(),
            // Enable delta compression when replicating the trail
            // TODO: delta-compression doesn't work. Is it because the client says that it acks
            //  some server messages, but it wasn't synced so it actually didn't handle those messages?
            DeltaCompression::<Trail>::default(),
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

fn color_from_client_id(client_id: u64) -> Color {
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
