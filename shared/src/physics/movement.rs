use crate::network::inputs::PlayerMovement;
use crate::physics::FixedSet;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::client::prediction::Predicted;
use lightyear::prelude::*;
use crate::player::bike::BIKE_VELOCITY;

const TURN_SPEED: f32 = 5.0;

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
        app.add_systems(FixedUpdate, (turn_from_inputs, update_velocity).in_set(FixedSet::HandleInputs));
    }
}

/// System that takes the 'rotation' from the input and uses it to turn the bikes
fn turn_from_inputs(
    mut query: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &ActionState<PlayerMovement>,
        ),
        // apply inputs either on predicted entities on the client, or replicating entities on the server
        Or<(With<Predicted>, With<Replicating>)>,
    >,
) {
    for (mut linear, mut angular, mut action_state) in query.iter_mut() {
        // angle in radians between the cursor and the bike
        let angle = action_state.value(&PlayerMovement::Rotate);

        let degrees = angle * 180.0 / std::f32::consts::PI;
        if degrees.abs() > 10.0 {
            angular.0 = -TURN_SPEED * angle.signum();
        } else {
            angular.0 = -TURN_SPEED * angle.signum() * (degrees.abs() / 30.0);
        }
        info!("Turning the bike as cursor is  {degrees:?} degrees from bike rotation. Turn speed: {angular:?}");


        // *rotation = *rotation * Rotation::radians(angle);
        // velocity.0 = Vec2::from_angle(angle).rotate(velocity.0);
    }
}


/// Move the bikes towards where they are looking
fn update_velocity(
    mut query: Query<(&Rotation, &mut LinearVelocity),
        // apply inputs either on predicted entities on the client, or replicating entities on the server
        (Or<(With<Predicted>, With<Replicating>)>, Changed<Rotation>)
        >,
) {
    for (rot, mut linear) in query.iter_mut() {
        linear.0 = Vec2::new(rot.cos, rot.sin) * BIKE_VELOCITY;
    }
}
