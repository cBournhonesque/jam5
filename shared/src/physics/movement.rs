use crate::network::inputs::PlayerMovement;
use crate::physics::FixedSet;
use crate::player::bike::{ACCEL, BASE_SPEED, DRAG, FAST_SPEED, FAST_SPEED_MAX_SPEED_DISTANCE};
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::client::prediction::Predicted;
use lightyear::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                // make sure that any physics simulation happens after the HandleInputs SystemSet
                // (where we apply user's actions)
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(FixedSet::Physics),
                (FixedSet::HandleInputs, FixedSet::Physics).chain(),
            ),
        );
        app.add_systems(
            FixedUpdate,
            (move_bike_system).in_set(FixedSet::HandleInputs),
        );
    }
}

/// System that takes the 'rotation' from the input and uses it to turn the bikes
fn move_bike_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &mut Rotation,
            &mut LinearVelocity,
            &ActionState<PlayerMovement>,
        ),
        // apply inputs either on predicted entities on the client, or replicating entities on the server
        Or<(With<Predicted>, With<Replicating>)>,
    >,
) {
    for (mut rotation, mut linear, action_state) in query.iter_mut() {
        // the wish_dir is the direction we wish to move in
        let wish_dir = action_state
            .axis_pair(&PlayerMovement::WishDir)
            .unwrap_or_default()
            .xy()
            .normalize_or_zero();

        // speed we wish to move at is based on mouse distance
        let mouse_distance = action_state.value(&PlayerMovement::MouseDistance);
        let normalized_mouse_distance =
            (mouse_distance / FAST_SPEED_MAX_SPEED_DISTANCE).clamp(0.0, 1.0);
        let wish_speed = BASE_SPEED.lerp(FAST_SPEED, normalized_mouse_distance);

        let mut current_velocity = linear.0;

        current_velocity = apply_drag(
            current_velocity,
            current_velocity.length(),
            DRAG,
            fixed_time.delta_seconds(),
        );

        current_velocity += accelerate(
            wish_dir,
            wish_speed,
            current_velocity.dot(wish_dir),
            ACCEL,
            fixed_time.delta_seconds(),
        );

        linear.0 = current_velocity;

        // rotate towards the velocity
        if current_velocity.length_squared() > 0.01 {
            let target_rotation = current_velocity.y.atan2(current_velocity.x);
            rotation.sin = target_rotation.sin();
            rotation.cos = target_rotation.cos();
        }
    }
}

fn apply_drag(velocity: Vec2, current_speed: f32, drag: f32, delta_seconds: f32) -> Vec2 {
    let mut new_velocity = velocity;

    let speed = current_speed;
    if speed > 0.0 {
        let drag_force = drag * speed;
        new_velocity -= new_velocity.normalize_or_zero() * drag_force * delta_seconds;
    }

    new_velocity
}

// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/bg_pmove.c#L240
fn accelerate(
    wish_dir: Vec2,
    wish_speed: f32,
    current_speed: f32,
    accel: f32,
    delta_seconds: f32,
) -> Vec2 {
    let add_speed = wish_speed - current_speed;

    if add_speed <= 0.0 {
        return Vec2::ZERO;
    }

    let mut accel_speed = accel * delta_seconds * wish_speed;
    if accel_speed > add_speed {
        accel_speed = add_speed;
    }

    wish_dir * accel_speed
}
