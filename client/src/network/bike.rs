//! Module to handle the networking of bikes on the client side

use crate::render::zones::ZoneRenderMarker;
use crate::render::{trail::TrailRenderMarker, zones::ZoneMaterial};
use avian2d::prelude::{Position, RigidBody, Rotation};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_prototype_lyon::prelude::*;
use lightyear::prelude::client::*;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::zone::Zone;

pub struct BikeNetworkPlugin;

impl Plugin for BikeNetworkPlugin {
    fn build(&self, app: &mut App) {
        // run this after the Predicted bike gets spawned and has its components synced,
        app.add_systems(
            PreUpdate,
            handle_new_predicted_bike.after(PredictionSet::SpawnHistory),
        );
        app.add_systems(
            PreUpdate,
            handle_new_confirmed_bike.after(PredictionSet::SpawnPrediction),
        );
    }
}

/// When a predicted bike gets created, we want to:
/// - add VisualInterpolateStatus component to visually interpolate the bike's Position/Rotation in Update
/// between two FixedUpdate values
/// - add RigidBody::Kinematic component so that the bike is affected by physics
fn handle_new_predicted_bike(
    mut commands: Commands,
    predicted_bikes: Query<
        Entity,
        (
            With<BikeMarker>,
            With<Predicted>,
            Without<VisualInterpolateStatus<Position>>,
        ),
    >,
) {
    for entity in predicted_bikes.iter() {
        commands.entity(entity).insert((
            VisualInterpolateStatus::<Position>::default(),
            VisualInterpolateStatus::<Rotation>::default(),
            RigidBody::Kinematic,
        ));
    }
}

/// When a confirmed bike gets created, we want to:
/// - create a new entity that will hold the zone mesh to render
fn handle_new_confirmed_bike(
    mut commands: Commands,
    mut materials: ResMut<Assets<ZoneMaterial>>,
    confirmed_bikes: Query<(Entity, &ColorComponent), (With<BikeMarker>, Added<Confirmed>)>,
) {
    for (entity, color) in confirmed_bikes.iter() {
        // color values above 1.0 enable bloom
        let zone_color: Color = (Color::Hsva(Hsva {
            // saturation: 0.2,
            ..Hsva::from(color.0)
        })
        .to_linear()
            * 3.0)
            .into();
        let trail_color: Color = (color.0.to_linear() * 10.0).into();

        commands.entity(entity).with_children(|parent| {
            // add the entity that will hold the zone mesh
            parent
                .spawn((
                    Path::default(),
                    MaterialMesh2dBundle {
                        material: materials.add(ZoneMaterial::new(zone_color, None)),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    ZoneRenderMarker,
                    NoFrustumCulling,
                    Fill::color(zone_color),
                ))
                .insert(Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)));
            // add the entity that will hold the trail mesh
            parent.spawn((
                ShapeBundle::default(),
                TrailRenderMarker,
                NoFrustumCulling,
                Stroke::new(trail_color, 1.0),
            ));
        });
    }
}
