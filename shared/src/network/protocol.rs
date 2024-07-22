//! Defines the shared network protocol between the client and server

use crate::network::inputs::PlayerMovement;
use crate::player::bike::{BikeMarker, ColorComponent};
use crate::player::trail::Trail;
use crate::player::zone::ZoneManager;
use crate::player::Player;
use avian2d::prelude::*;
use bevy::app::{App, Plugin};
use bevy::prelude::default;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use lightyear::utils::avian2d::*;

pub struct ProtocolPlugin;

#[derive(Channel)]
pub struct Channel1;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Channels

        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerMovement>::default());

        // Messages

        // Components
        app.register_component::<Player>(ChannelDirection::ServerToClient);

        app.register_component::<ColorComponent>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<BikeMarker>(ChannelDirection::ServerToClient)
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

        app.register_component::<Trail>(ChannelDirection::ServerToClient);
        app.register_resource::<ZoneManager>(ChannelDirection::ServerToClient);

        // channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
