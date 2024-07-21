use bevy::app::{App, Plugin};

pub mod player;

pub(crate) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerRenderPlugin);
    }
}
