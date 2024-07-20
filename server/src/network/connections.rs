//! Handle client connections

use bevy::prelude::*;
use bevy::reflect::ApplyError;
use lightyear::prelude::*;
use lightyear::prelude::server::*;
use shared::player::bike::BikeBundle;


/// Spawn a new bike when a player connects
fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    let client_id = trigger.event().client_id;
    commands.spawn((
        BikeBundle::new_at(Vec2::new(0.0, 0.0)),
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