//! Defines the shared network protocol between the client and server

use crate::network::inputs::PlayerMovement;
use crate::network::message::{KillMessage, KilledByMessage};
use crate::player::bike::{BikeMarker, ColorComponent};
use crate::player::scores::{Score, Stats};
use crate::player::trail::Trail;
use crate::player::zone::Zones;
use avian2d::prelude::*;
use bevy::app::{App, Plugin};
use bevy::prelude::{default, Name};
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use lightyear::utils::avian2d::*;

pub struct ProtocolPlugin;

#[derive(Channel)]
pub struct Channel1;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerMovement>::default());

        // Messages
        app.register_message::<KilledByMessage>(ChannelDirection::ServerToClient)
            .add_map_entities();
        app.register_message::<KillMessage>(ChannelDirection::ServerToClient)
            .add_map_entities();

        // Components
        app.register_component::<Score>(ChannelDirection::ServerToClient);
        app.register_component::<Stats>(ChannelDirection::ServerToClient);

        app.register_component::<ColorComponent>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<BikeMarker>(ChannelDirection::ServerToClient)
            // .add_map_entities()
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Position>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp);
        // TODO: remove correction for now to make rollbacks more obvious
        // .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp);
        // TODO: remove correction for now to make rollbacks more obvious
        // .add_correction_fn(rotation::lerp);

        // NOTE: interpolation/correction is only needed for components that are visually displayed!
        // we still need prediction to be able to correctly predict the physics on the client
        app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Name>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Trail>(ChannelDirection::ServerToClient)
            .add_delta_compression();
        app.register_component::<Zones>(ChannelDirection::ServerToClient)
            .add_delta_compression();
    }
}
