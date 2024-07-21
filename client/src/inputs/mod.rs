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
        app.add_systems(
            PreUpdate,
            // make sure this runs after the other leafwing systems
            // mouse_to_world_space.in_set(InputManagerSystem::ManualControl),
            capture_input.in_set(InputManagerSystem::ManualControl),
        );
        // TODO: ideally use an observer? this should only run once
        app.add_systems(Update, add_input_map);
    }
}

fn add_input_map(
    mut commands: Commands,
    // TODO: put the InputMap on the bike entity or the player entity?
    predicted_players: Query<
        Entity,
        (
            With<Predicted>,
            With<BikeMarker>,
            Without<InputMap<PlayerMovement>>,
        ),
    >,
) {
    for entity in predicted_players.iter() {
        commands
            .entity(entity)
            .insert(InputMap::<PlayerMovement>::default());
    }
}

fn capture_input(
    mut action_state_query: Query<(&Position, &mut ActionState<PlayerMovement>), With<Predicted>>,
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

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((bike_pos, mut action_state)) = action_state_query.get_single_mut() {
            let wish_dir = world_position - bike_pos.0;
            action_state.press(&PlayerMovement::WishDir);
            action_state
                .action_data_mut(&PlayerMovement::WishDir)
                .unwrap()
                .axis_pair = Some(DualAxisData::from_xy(wish_dir));

            let mouse_distance = bike_pos.0.distance(world_position);
            action_state.press(&PlayerMovement::MouseDistance);
            action_state
                .action_data_mut(&PlayerMovement::MouseDistance)
                .unwrap()
                .value = mouse_distance;
        }
    }
}
