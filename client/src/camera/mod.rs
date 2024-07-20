use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::transform::TransformSystem;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::client::{InterpolationSet, Predicted};
use shared::network::protocol::prelude::TailPoints;
use crate::inputs::LocalInput;
use crate::network::inputs::Owned;

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
        // state
        app.init_state::<CameraState>();
        // follow
        app.add_systems(OnEnter(CameraState::Follow), enter_follow_camera);

        // full
        app.add_systems(OnEnter(CameraState::Full), enter_full_camera);

        // we could run during update, because the predicted movement is updated in FixedUpdate
        app.add_systems(PostUpdate, (toggle_camera, follow_camera
            .before(TransformSystem::TransformPropagate)
            .after(InterpolationSet::VisualInterpolation)
            .run_if(in_state(CameraState::Follow))));

    }
}


fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn toggle_camera(
    mut next_state: ResMut<NextState<CameraState>>,
    current_state: Res<State<CameraState>>,
    query: Query<&ActionState<LocalInput>, With<Owned>>,
) {
    if let Ok(action_state) = query.get_single() {
        if action_state.just_pressed(&LocalInput::ToggleCamera) {
            match current_state.get() {
                CameraState::Follow => next_state.set(CameraState::Full),
                CameraState::Full => next_state.set(CameraState::Follow),
            }
        }
    }
}

/// TODO: this kill thing is too complicated, set HasFocusHead component on the player
/// entity once and then multiple systems can depend on HasFocusHead (follow camera, sounds, scope, etc.)
/// also server can use HasFocusHead.
///
/// System to make the camera follow the head of the player, or the head of the killer
fn follow_camera(
    predicted: Query<&TailPoints, With<Predicted>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    // how much we stick to the new position
    // let lerp = 0.1;
    // let lerp = 1.0;
    if let Ok(mut camera_pos) = camera_query.get_single_mut() {
        if let Ok(pos) = predicted.get_single() {
            let head = pos.front().0;
            // *camera_pos = Transform::from_translation(camera_pos.translation.mul_add(Vec3::splat(1.0 - lerp), Vec3::from((head, 0.0)) * lerp));
            *camera_pos = Transform::from_xyz(head.x, head.y, 0.0);
        }
    }
    // player is dead: camera follows killer's head
}


/// Switch camera to follow view, reset the projection
fn enter_follow_camera(mut camera_query: Query<&mut OrthographicProjection, With<Camera>>) {
    if let Ok(mut projection) = camera_query.get_single_mut() {
        // NOTE: do not set the window size to >1.0 as this can cause jitters due to fractional pixel movement
        projection.scaling_mode = ScalingMode::WindowSize(1.0);
        projection.scale = 1.0;
    }
}

/// Switch camera to full view, reset the projection
fn enter_full_camera(mut camera_query: Query<&mut OrthographicProjection, With<Camera>>) {
    if let Ok(mut projection) = camera_query.get_single_mut() {
        projection.scaling_mode = ScalingMode::WindowSize(1.0);
        projection.scale = 1.0;
    }
}