#[cfg(feature = "dev")]
pub mod debug;
pub mod map;
pub mod network;

pub mod player;

pub mod physics;

use bevy::log::{Level, LogPlugin};
use bevy::state::app::StatesPlugin;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};

pub struct SharedPlugin {
    pub headless: bool,
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        if self.headless {
            app.add_plugins((
                MinimalPlugins,
                StatesPlugin,
                LogPlugin {
                    level: Level::INFO,
                    // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
                    // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::client::prediction::rollback=debug".to_string(),
                    // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::shared::replication::send=info,lightyear::shared::replication::delta=info,lightyear::protocol::component=info",
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::shared::replication::send=warn,lightyear::shared::replication::delta=warn,lightyear::protocol::component=warn"
                        .to_string(),
                    ..default()
                },
            ));
        } else {
            app.add_plugins(
                DefaultPlugins
                    .set(LogPlugin {
                        level: Level::INFO,
                        // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
                        // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::client::prediction::rollback=debug".to_string(),
                        // filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::shared::replication::send=info,lightyear::shared::replication::delta=info,lightyear::protocol::component=info"
                        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,lightyear::shared::replication::send=warn,lightyear::shared::replication::delta=warn,lightyear::protocol::component=warn"
                            .to_string(),
                        ..default()
                    })
                    .set(AssetPlugin {
                        file_path: "assets".to_string(),
                        // Wasm builds will check for meta files (that don't exist) if this isn't set.
                        // This causes errors and even panics on web build on itch.
                        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(WindowPlugin {
                        primary_window: Window {
                            title: "jam5".to_string(),
                            canvas: Some("#bevy".to_string()),
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: true,
                            ..default()
                        }
                        .into(),
                        ..default()
                    })
                    .set(AudioPlugin {
                        global_volume: GlobalVolume {
                            volume: Volume::new(0.3),
                        },
                        ..default()
                    }),
            );
        }

        #[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin);

        // Add shared game logic plugins
        app.add_plugins(map::MapPlugin);
        app.add_plugins(physics::PhysicsPlugin);
        app.add_plugins(player::bike::BikePlugin);
        app.add_plugins(player::death::DeathPlugin);
        app.add_plugins(player::trail::TrailPlugin);
        app.add_plugins(player::zone::ZonePlugin);
    }
}
