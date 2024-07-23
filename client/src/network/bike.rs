//! Module to handle the networking of bikes on the client side

use crate::render::trail::TrailRenderMarker;
use crate::render::zones::ZoneRenderMarker;
use avian2d::prelude::{Position, RigidBody, Rotation};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_prototype_lyon::prelude::*;
use lightyear::client;
use lightyear::connection::netcode::Client;
use lightyear::prelude::{client::*, ClientId, MainSet};
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::zone::{self, Zone, Zones};

pub struct BikeNetworkPlugin;

impl Plugin for BikeNetworkPlugin {
    fn build(&self, app: &mut App) {
        // run this after the Predicted bike gets spawned and has its components synced,
        app.add_systems(
            PreUpdate,
            handle_new_predicted_bike.after(PredictionSet::SpawnHistory),
        );
        app.add_systems(PreUpdate, handle_new_confirmed_bike.after(MainSet::Receive));
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
    // checking with Added<BikeMarker> ensures that we only run this system once for each new bike, including other people's bikes and "mock bikes" for testing
    confirmed_bikes: Query<(Entity, &BikeMarker, &ColorComponent, &Zones), Added<BikeMarker>>,
) {
    for (entity, bike, color, zones) in confirmed_bikes.iter() {
        info!("Decorating bike with color: {:?}", color.0);

        // color values above 1.0 enable bloom
        let c = color.0.to_linear();
        let zone_fill_color: Color = Color::srgba(c.red, c.green, c.blue, 0.15);
        let zone_border_color: Color = (c * 2.0).into();
        let trail_color: Color = (c * 10.0).into();
        let zone_z_order = -(bike.client_id as f32) * 100.0;

        commands.entity(entity).with_children(|parent| {
            // add the entity that will hold the zone mesh
            parent
                .spawn((
                    ShapeBundle::default(),
                    ZoneRenderMarker,
                    NoFrustumCulling,
                    Fill::color(zone_fill_color),
                    Stroke::new(zone_border_color, 4.0),
                ))
                .insert(GlobalTransform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    -zone_z_order,
                )))
                .insert(Path::from(zones));

            // add the entity that will hold the trail mesh
            parent.spawn((
                ShapeBundle::default(),
                TrailRenderMarker,
                NoFrustumCulling,
                Stroke::new(trail_color, 2.0),
            ));
        });
    }
}
