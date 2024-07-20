//! Defines the shared network protocol between the client and server

use avian2d::prelude::*;
use bevy::app::{App, Plugin};
use lightyear::prelude::*;
use lightyear::prelude::client::*;
use lightyear::utils::avian2d::*;
use crate::network::inputs::PlayerMovement;
use crate::player::Player;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Channels

        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerMovement>::default());

        // Messages

        // Components
        app.register_component::<Player>(ChannelDirection::ServerToClient);

        app.register_component::<Position>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        // NOTE: interpolation/correction is only needed for components that are visually displayed!
        // we still need prediction to be able to correctly predict the physics on the client
        app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);
    }
}