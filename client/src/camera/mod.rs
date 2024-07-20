use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum CameraState {
    // follow the player
    #[default]
    Follow,
    // view the full map
    Full,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
    }
}


fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("Camera")));
}