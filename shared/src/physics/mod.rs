pub mod movement;
pub mod util;

use crate::network::config::FIXED_TIMESTEP_HZ;
use crate::physics::movement::MovementPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

/// FixedUpdate system sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    HandleInputs,
    // apply physics steps
    Physics,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // PLUGINS
        app.add_plugins(
            avian2d::PhysicsPlugins::new(FixedUpdate)
                .build()
                .disable::<ColliderHierarchyPlugin>(),
        );
        app.add_plugins(MovementPlugin);
        // RESOURCES
        app.insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
