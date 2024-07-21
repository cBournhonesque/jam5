use avian2d::position::Position;
use bevy::{prelude::*, window::PrimaryWindow};
use lightyear::client::prediction::Predicted;
use shared::player::bike::BikeMarker;

pub const FOLLOW_CAMERA_Z: f32 = 2.0;
pub const CAMERA_FOLLOW_SPEED: f32 = 5.0;

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
        app.add_systems(Update, update_camera);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("Camera")));
}

fn update_camera(
    time: Res<Time>,
    mut q_camera: Query<(&Camera, &mut Transform, &GlobalTransform), With<Camera2d>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_player: Query<&Position, (With<BikeMarker>, With<Predicted>)>,
) {
    let window = q_window.single();

    if let Ok((camera, mut cam_xform, cam_gxform)) = q_camera.get_single_mut() {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(cam_gxform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(player_pos) = q_player.iter().next() {
                let target = (player_pos.0 + world_position) * 0.5;
                let current_pos = cam_xform.translation.truncate();
                let new_pos = current_pos.lerp(target, CAMERA_FOLLOW_SPEED * time.delta_seconds());
                let new_pos_3d = Vec3::new(new_pos.x, new_pos.y, FOLLOW_CAMERA_Z);
                cam_xform.translation = new_pos_3d;
            }
        }
    }
}
