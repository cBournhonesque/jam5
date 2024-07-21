//! Module to handle the networking of bikes on the client side

use avian2d::prelude::{Position, RigidBody, Rotation};
use bevy::prelude::*;
use lightyear::prelude::client::*;
use shared::player::bike::BikeMarker;

pub struct BikeNetworkPlugin;


impl Plugin for BikeNetworkPlugin {
    fn build(&self, app: &mut App) {
        // run this after the Predicted bike gets spawned and has its components synced,
        app.add_systems(PreUpdate, handle_new_predicted_bike.after(PredictionSet::SpawnHistory));
    }
}

/// When a predicted bike gets created, we want to:
/// - add VisualInterpolateStatus component to visually interpolate the bike's Position/Rotation in Update
/// between two FixedUpdate values
/// - add RigidBody::Kinematic component so that the bike is affected by physics
fn handle_new_predicted_bike(
    mut commands: Commands,
    predicted_bikes: Query<Entity, (With<BikeMarker>, With<Predicted>, Without<VisualInterpolateStatus<Position>>)>,
) {
    for entity in predicted_bikes.iter() {
        commands.entity(entity)
            .insert((
                    VisualInterpolateStatus::<Position>::default(),
                    VisualInterpolateStatus::<Rotation>::default(),
                    RigidBody::Kinematic
            ));
    }
}