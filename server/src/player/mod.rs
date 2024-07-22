use bevy::prelude::*;

mod trail;
mod zone;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(trail::TrailPlugin);
        app.add_plugins(zone::ZonePlugin);
    }
}
