use bevy::prelude::*;

mod trail;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(trail::TrailPlugin);
    }
}
