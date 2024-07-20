//! Handle client connections

use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy::reflect::ApplyError;
use lightyear::prelude::*;
use lightyear::prelude::server::*;
use shared::player::bike::BikeBundle;


/// Spawn a new bike when a player connects
pub(crate) fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    info!("Spawning bike for client {}", trigger.event().client_id);
    let client_id = trigger.event().client_id;
    commands.spawn((
        BikeBundle::new_at(Vec2::new(0.0, 0.0)),
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
            ..default()
        },
    ));
}