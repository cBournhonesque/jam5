use avian2d::position::{Position, Rotation};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::*;
use lightyear::prelude::client::*;
use crate::player::bike::BikeMarker;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedPostUpdate, post_fixed_update_log);
        // app.add_systems(Last, last_log);

        if app.is_plugin_added::<RenderPlugin>() {
            app.add_plugins(WorldInspectorPlugin::default());
        }
    }
}

pub(crate) fn post_fixed_update_log(
    time: Res<TimeManager>,
    tick_manager: Res<TickManager>,
    players: Query<
        (
            Entity,
            &Position,
            Option<&VisualInterpolateStatus<Position>>,
            // &Rotation,
            // Option<&Correction<Position>>,
            // Option<&Correction<Rotation>>,
        ),
        (With<BikeMarker>, Without<Confirmed>, Without<Interpolated>),
    >,
) {
    let tick = tick_manager.tick();
    for (entity, position, visual_position) in players.iter() {
        info!(?tick, ?position, ?visual_position, overstep = ?time.overstep(), "Player in POST FIXED UPDATE");
    }
}

pub(crate) fn last_log(
    time: Res<TimeManager>,
    tick_manager: Res<TickManager>,
    players: Query<
        (
            Entity,
            &Position,
            &Rotation,
            // Option<&Correction<Position>>,
            // Option<&Correction<Rotation>>,
        ),
        (With<BikeMarker>, Without<Confirmed>, Without<Interpolated>),
    >,
) {
    let tick = tick_manager.tick();
    for (entity, position, rotation) in players.iter() {
        info!(?tick, ?position, overstep = ?time.overstep(), "Player in LAST");
        trace!(?tick, ?entity, ?position, rotation = ?rotation.as_degrees(), "Player in LAST");
    }
}