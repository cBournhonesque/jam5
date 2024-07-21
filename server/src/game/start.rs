use bevy::prelude::*;
use shared::map::SpawnMap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_start);
    }
}

fn game_start(mut commands: Commands) {
    // spawn the map
    commands.trigger(SpawnMap);
}
