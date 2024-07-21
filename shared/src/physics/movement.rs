use crate::network::inputs::PlayerMovement;
use crate::physics::FixedSet;
use crate::player::bike::{
    ACCEL, BASE_SPEED, DRAG, FAST_DRAG, FAST_SPEED, FAST_SPEED_MAX_SPEED_DISTANCE,
    MAX_ROTATION_SPEED,
};
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
    tick_manager: Res<TickManager>,
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
        let delta = fixed_time.delta_seconds();
        let tick = tick_manager.tick();

        // speed we wish to move at is based on mouse distance
        if let Some(relative_mouse_pos) =
            action_state.axis_pair(&PlayerMovement::MousePositionRelative)
        {
            let wish_dir = relative_mouse_pos.xy().normalize_or_zero();
            let normalized_mouse_distance =
                (relative_mouse_pos.length() / FAST_SPEED_MAX_SPEED_DISTANCE).clamp(0.0, 1.0);
            let wish_speed = BASE_SPEED.lerp(FAST_SPEED, normalized_mouse_distance);
            let wish_drag = DRAG.lerp(FAST_DRAG, normalized_mouse_distance);

            // limit the rotation
            let current_dir = Vec2::new(rotation.cos, rotation.sin);
            let angle_diff = current_dir.angle_between(wish_dir);
            let max_rotation = MAX_ROTATION_SPEED * delta;
            let limited_rotation = angle_diff.clamp(-max_rotation, max_rotation);
            let actual_dir = current_dir.rotate(Vec2::from_angle(limited_rotation));

            let mut current_velocity = linear.0;

            // for simplicity, use a constant speed for now
            //current_velocity = actual_dir * BASE_SPEED;

            // TODO: maybe add drag/acceleration/speed based on mouse distance?
            current_velocity = apply_drag(
                current_velocity,
                current_velocity.length(),
                wish_drag,
                delta,
            );
            current_velocity += accelerate(
                actual_dir,
                wish_speed,
                current_velocity.dot(actual_dir),
                ACCEL,
                delta,
            );

            linear.0 = current_velocity;

            // rotate towards the direction of movement
            let new_rotation = actual_dir.y.atan2(actual_dir.x);
            rotation.sin = new_rotation.sin();
            rotation.cos = new_rotation.cos();

            trace!(
                ?tick,
                ?delta,
                ?relative_mouse_pos,
                ?linear,
                ?rotation,
                "Moving bike from input"
            );
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
