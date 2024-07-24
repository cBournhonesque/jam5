use crate::map::MAP_SIZE;
use crate::network::inputs::PlayerMovement;
use crate::physics::FixedSet;
use crate::player::bike::{
    BikeMarker, ClientIdMarker, ACCEL, BASE_SPEED, DRAG, FAST_DRAG, FAST_SPEED,
    FAST_SPEED_MAX_SPEED_DISTANCE, MAX_ROTATION_SPEED, OUR_ZONE_SPEED_MULTIPLIER,
};
use crate::player::zone::Zones;
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
    // TODO: add spatial index
    q_zones: Query<(&Zones, &ClientIdMarker)>,
    mut q_bike: Query<
        (
            // TODO: do we need this?
            &ClientIdMarker,
            &mut Position,
            &mut Rotation,
            &mut LinearVelocity,
            &ActionState<PlayerMovement>,
        ),
        // apply inputs either on predicted entities on the client, or replicating entities on the server
        Or<(With<Predicted>, With<Replicating>)>,
    >,
) {
    let mut zones = q_zones.iter();
    for (client_id, mut position, mut rotation, mut linear, action_state) in q_bike.iter_mut() {
        let delta = fixed_time.delta_seconds();
        let tick = tick_manager.tick();

        // speed we wish to move at is based on mouse distance
        if let Some(relative_mouse_pos) =
            action_state.axis_pair(&PlayerMovement::MousePositionRelative)
        {
            let wish_dir = relative_mouse_pos.xy().normalize_or_zero();
            let normalized_mouse_distance =
                (relative_mouse_pos.length() / FAST_SPEED_MAX_SPEED_DISTANCE).clamp(0.0, 1.0);

            // are we in our own zone?
            let wish_speed_multiplier =
                if zones.any(|(z, zone_owner)| zone_owner == client_id && z.contains(position.0)) {
                    OUR_ZONE_SPEED_MULTIPLIER
                } else {
                    1.0
                };

            let wish_speed =
                BASE_SPEED.lerp(FAST_SPEED, normalized_mouse_distance) * wish_speed_multiplier;
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

            // rotate towards the direction of movement
            let new_rotation = actual_dir.y.atan2(actual_dir.x);
            rotation.sin = new_rotation.sin();
            rotation.cos = new_rotation.cos();

            // map bounds
            let deproject_padding = 10.0;
            let iso_ratio = 0.866;
            let a = MAP_SIZE;
            let b = MAP_SIZE * iso_ratio;

            if (position.0.x.powi(2) / a.powi(2)) + (position.0.y.powi(2) / b.powi(2)) > 1.0 {
                // deproject
                let scale = ((position.0.x.powi(2) / a.powi(2))
                    + (position.0.y.powi(2) / b.powi(2)))
                .sqrt();
                position.0 = Vec2::new(position.0.x / scale, position.0.y / scale);

                // calculate normal and tangent
                let normal =
                    Vec2::new(position.0.x / a.powi(2), position.0.y / b.powi(2)).normalize();
                let tangent = Vec2::new(-normal.y, normal.x);

                // slide
                let slide_velocity = tangent * current_velocity.dot(tangent);
                let normal_velocity = normal * current_velocity.dot(normal);
                current_velocity = slide_velocity - normal_velocity * 0.75;

                // inward facing rotation so we dont get stuck along the wall
                let inward_factor = 1.5; // how much we're facing inward
                let target_direction = (tangent - normal * inward_factor).normalize();
                let new_rotation = target_direction.y.atan2(target_direction.x);

                rotation.cos = new_rotation.cos();
                rotation.sin = new_rotation.sin();
            }

            linear.0 = current_velocity;

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
