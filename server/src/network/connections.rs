//! Handle client connections

use avian2d::prelude::{Position, RigidBody};
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::HashSet;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use rand::Rng;
use shared::network::message::SpawnPlayerMessage;
use shared::player::bike::{BikeBundle, BikeMarker};
use shared::player::trail::{Trail, TrailBundle};
use shared::player::zone::{Zones, ZonesBundle};
use std::time::Duration;

#[derive(Resource)]
pub struct AvailableColors(pub Vec<Color>);

impl Default for AvailableColors {
    fn default() -> Self {
        Self(Vec::from([
            css::LIMEGREEN.into(),
            css::PINK.into(),
            css::YELLOW.into(),
            css::AQUA.into(),
            css::CRIMSON.into(),
            css::GOLD.into(),
            css::ORANGE_RED.into(),
            css::SILVER.into(),
            css::SALMON.into(),
            css::YELLOW_GREEN.into(),
            css::WHITE.into(),
            css::RED.into(),
        ]))
    }
}

impl AvailableColors {
    pub fn pick_color(&mut self) -> Color {
        let index = rand::thread_rng().gen_range(0..self.0.len());
        self.0.swap_remove(index)
    }

    pub fn add_color(&mut self, color: Color) {
        self.0.push(color);
    }
}

/// Spawn a new bike when a player connects, along with a `Trail` and a `Zones` entities
pub(crate) fn spawn_bike(
    mut messages: ResMut<Events<MessageEvent<SpawnPlayerMessage>>>,
    mut colors: ResMut<AvailableColors>,
    mut commands: Commands,
) {
    for message in messages.drain() {
        let client_id = message.context;
        let name = message.message.name;

        info!(
            "Spawning bike for client {:?}, player {:?}",
            client_id, name
        );
        let color = colors.pick_color();
        let pos = Vec2::new(0.0, 0.0);

        // NOTE: for complicated reasons related to lightyear:
        //  - each entity must be replicated in a different replication group (so that delta compression works)
        //  - but the trail/zones must be replicated after the bike, so that the ParentSync has a pointer to the correct entities
        //
        // As a solution, we will replicate bike/trail/zone without replicating the hierarchy
        // We will add the hierarchy manually on the client side by comparing client ids
        let bike = commands
            .spawn((
                BikeBundle::new_at(client_id, name, pos, color),
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
            ))
            // do not replicate the hierarchy at all, because the ParentSync component might be invalid
            // instead we will build the hierarchy on the client side manually
            .remove::<ReplicateHierarchy>()
            .id();

        let trail = commands
            .spawn((
                TrailBundle::new_at(pos, client_id),
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
            .remove::<(ReplicateHierarchy, SyncTarget)>()
            .id();
        //
        let zones = commands
            .spawn((
                ZonesBundle::new(client_id),
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
            .remove::<(ReplicateHierarchy, SyncTarget)>()
            .id();
        commands.entity(bike).add_child(trail).add_child(zones);
    }
}
