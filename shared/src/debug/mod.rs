use avian2d::position::{Position, Rotation};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::*;
use lightyear::prelude::client::*;
use lightyear::shared::replication::delta::{DeltaComponentHistory, DeltaManager};
use crate::player::bike::BikeMarker;
use crate::player::trail::Trail;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedPostUpdate, post_fixed_update_bike_log);
        // app.add_systems(Last, last_bike_log);
        app.add_systems(FixedUpdate, fixed_update_trail_log);
        // app.add_systems(FixedUpdate, delta_manager_log);

        if app.is_plugin_added::<RenderPlugin>() {
            app.add_plugins(WorldInspectorPlugin::default());
        }
    }
}

fn delta_manager_log(
    manager: Option<Res<ServerConnectionManager>>,
) {
    if let Some(manager) = manager {
        info!(?manager.delta_manager, "Delta Manager");
    }
}

pub(crate) fn post_fixed_update_bike_log(
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

pub(crate) fn last_bike_log(
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

pub(crate) fn fixed_update_trail_log(
    tick_manager: Res<TickManager>,
    trails: Query<
        (
            Entity,
            &Trail,
            Option<&DeltaComponentHistory<Trail>>,
        ),
    >,
) {
    let tick = tick_manager.tick();
    for (entity, trail, trail_history) in trails.iter() {
        info!(?tick, ?entity, ?trail_history, "Trails in LAST");
    }
}