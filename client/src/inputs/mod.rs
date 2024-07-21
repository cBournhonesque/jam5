use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::client::*;
use shared::network::inputs::PlayerMovement;
use shared::player::bike::BikeMarker;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate,
            // make sure this runs after the other leafwing systems
            // mouse_to_world_space.in_set(InputManagerSystem::ManualControl),
                        mouse_to_rotation.in_set(InputManagerSystem::ManualControl),
        );
        // TODO: ideally use an observer? this should only run once
        app.add_systems(Update, add_input_map);
    }
}


fn add_input_map(
    mut commands: Commands,
    // TODO: put the InputMap on the bike entity or the player entity?
    predicted_players: Query<Entity, (With<Predicted>, With<BikeMarker>, Without<InputMap<PlayerMovement>>)>,
) {
    for entity in predicted_players.iter() {
        commands.entity(entity).insert(InputMap::<PlayerMovement>::default());
    }
}

/// Compute how the bike entity should rotate based on the cursor position
///
fn mouse_to_rotation(
    mut action_state_query: Query<(&Position, &Rotation, &mut ActionState<PlayerMovement>), With<Predicted>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        error!("Expected to find only one camera");
        return;
    };
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((pos, rot, mut action_state)) = action_state_query.get_single_mut() {
            // TODO: only accept the cursor position if it's "in front" of the bike
            // compute the angle between the bike and the cursor
            let diff = world_position - pos.0;
            let angle = diff.angle_between(Vec2::new(rot.cos, rot.sin));
            trace!("angle between cursor and bike on client: {}", angle);
            // positive angles mean we rotate to the left, negative to the right
            action_state.press(&PlayerMovement::Rotate);
            action_state.action_data_mut(&PlayerMovement::Rotate).unwrap().value = angle;
        }
    }
}


/// Sets the ActionState to the current cursor position in world space
fn mouse_to_world_space(
    mut action_state_query: Query<&mut ActionState<PlayerMovement>, With<Predicted>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        error!("Expected to find only one camera");
        return;
    };
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok(mut action_state) = action_state_query.get_single_mut() {
            action_state.press(&PlayerMovement::MoveCursor);
            action_state.action_data_mut(&PlayerMovement::MoveCursor).unwrap().axis_pair = Some(DualAxisData::from_xy(world_position));
        }
    }
}