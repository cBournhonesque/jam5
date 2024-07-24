use bevy::prelude::*;

mod trail;
pub mod death;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(trail::TrailPlugin);
        app.add_plugins(death::DeathPlugin);
    }
}
