//! Module to handle the networking of bikes on the client side

use avian2d::prelude::{Position, RigidBody, Rotation};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use lightyear::prelude::client::*;
use shared::player::bike::{BikeMarker, ColorComponent};
use crate::render::zones::ZoneRenderMarker;

pub struct BikeNetworkPlugin;


impl Plugin for BikeNetworkPlugin {
    fn build(&self, app: &mut App) {
        // run this after the Predicted bike gets spawned and has its components synced,
        app.add_systems(PreUpdate, handle_new_predicted_bike.after(PredictionSet::SpawnHistory));
        app.add_systems(PreUpdate, handle_new_confirmed_bike.after(PredictionSet::SpawnPrediction));
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

/// When a confirmed bike gets created, we want to:
/// - create a new entity that will hold the zone mesh to render
fn handle_new_confirmed_bike(
    mut commands: Commands,
    confirmed_bikes: Query<(Entity, &ColorComponent), (With<BikeMarker>, Added<Confirmed>)>,
) {
    for (entity, color) in confirmed_bikes.iter() {
        let zone_color = Color::Hsva(Hsva {
            saturation: 0.2,
            ..Hsva::from(color.0)
        });
        commands.entity(entity)
            .with_children(|parent| {
                parent.spawn((
                    ShapeBundle::default(),
                    ZoneRenderMarker,
                    Fill::color(zone_color),
                    Stroke::new(Color::WHITE, 5.0),
                ));
            });
    }
}