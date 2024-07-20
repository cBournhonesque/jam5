use bevy::prelude::*;
use avian2d::prelude::*;
use crate::network::config::FIXED_TIMESTEP_HZ;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // PLUGINS
        app.add_plugins(avian2d::PhysicsPlugins::new(FixedUpdate)
            .build()
            .disable::<ColliderHierarchyPlugin>());
        // RESOURCES
        app.insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
            .insert_resource(Gravity(Vec2::ZERO));
    }
}