use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use lightyear::{
    connection::netcode::ClientId,
    prelude::{
        server::{ControlledBy, Replicate, SyncTarget},
        Channel, ChannelKind, NetworkTarget, ReplicateResourceExt,
    },
};
use shared::{
    map::SpawnMap,
    network::protocol::Channel1,
    player::{
        bike::{BikeBundle, BikeMarker, ColorComponent},
        zone::{Zone, Zones},
    },
};

use crate::network::connections::color_from_client_id;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_start);
    }
}

fn game_start(mut commands: Commands) {
    println!("Game starting!");
    // spawn the map
    commands.trigger(SpawnMap);

    commands
        .spawn((
            BikeBundle::new_at(3, Vec2::new(0.0, 0.0), color_from_client_id(3)),
            Replicate::default(),
        ))
        .insert(Zones {
            zones: vec![Zone {
                exterior: vec![
                    Vec2::new(-1000.0, -1000.0),
                    Vec2::new(1000.0, -1000.0),
                    Vec2::new(1000.0, 1000.0),
                    Vec2::new(-1000.0, 1000.0),
                    Vec2::new(-1000.0, -1000.0),
                ],
                interiors: vec![],
            }],
        });
}
