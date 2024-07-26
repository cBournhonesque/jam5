use avian2d::position::Position;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::{prelude::*, window::PrimaryWindow};
use lightyear::client::prediction::Predicted;
use shared::player::bike::BikeMarker;
use shared::player::death::Dead;

pub const FOLLOW_CAMERA_Z: f32 = 2.0;
pub const CAMERA_FOLLOW_SPEED: f32 = 5.0;

pub const CAMERA_SCALE: f32 = 1.0;

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
        // Update the camera position based on the bike's position
        // This system should run after TransformPropagate so that the camera follows the
        // Visual position of the bike
        app.add_systems(PostUpdate, update_camera.after(TransformPropagate));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            projection: OrthographicProjection {
                near: -1000.,
                far: 1000.,
                scale: CAMERA_SCALE,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        Name::new("Camera"),
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

fn update_camera(
    time: Res<Time>,
    mut q_camera: Query<(&Camera, &mut Transform, &GlobalTransform), With<Camera2d>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_player: Query<&Position, (With<BikeMarker>, With<Predicted>, Without<Dead>)>,
) {
    let window = q_window.single();

    if let Ok((camera, mut cam_xform, cam_gxform)) = q_camera.get_single_mut() {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(cam_gxform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(player_pos) = q_player.iter().next() {
                let target = player_pos.0.lerp(world_position, 0.25);
                let current_pos = cam_xform.translation.truncate();
                let new_pos = current_pos.lerp(target, CAMERA_FOLLOW_SPEED * time.delta_seconds());
                let new_pos_3d = Vec3::new(new_pos.x, new_pos.y, FOLLOW_CAMERA_Z);
                cam_xform.translation = new_pos_3d;
            }
        }
    }
}
