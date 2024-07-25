use avian2d::prelude::*;
use bevy::input::keyboard;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use lightyear::client::input::leafwing::InputSystemSet;
use lightyear::prelude::client::*;
use lightyear::prelude::TickManager;
use shared::network::inputs::PlayerMovement;
use shared::player::bike::BikeMarker;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            // make sure this runs after the other leafwing systems
            // mouse_to_world_space.in_set(InputManagerSystem::ManualControl),

            // make sure we update the ActionState before buffering them
            capture_input
                .before(InputSystemSet::BufferClientInputs)
                .run_if(not(is_in_rollback)), // .in_set(InputManagerSystem::ManualControl),
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
        // NOTE: uncomment this to be able to pause players during testing
        // .insert(InputMap::<PlayerMovement>::new([(
        //     PlayerMovement::Pause,
        //     KeyCode::Space,
        // )]));
    }
}

fn capture_input(
    tick_manager: Res<TickManager>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut action_state_query: Query<
        (&BikeMarker, &Position, &mut ActionState<PlayerMovement>),
        With<Predicted>,
    >,
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
        if let Ok((bike, bike_pos, mut action_state)) = action_state_query.get_single_mut() {
            let mouse_position_relative = world_position - bike_pos.0;
            action_state.press(&PlayerMovement::MousePositionRelative);
            action_state
                .action_data_mut(&PlayerMovement::MousePositionRelative)
                .unwrap()
                .axis_pair = Some(DualAxisData::from_xy(mouse_position_relative));
            trace!(tick = ?tick_manager.tick(), ?mouse_position_relative, "Relative mouse position");
        }
    }
}
