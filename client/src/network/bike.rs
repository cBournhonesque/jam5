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
use shared::player::trail::Trail;
use shared::player::zone::{self, Zone, Zones};

pub struct BikeNetworkPlugin;

impl Plugin for BikeNetworkPlugin {
    fn build(&self, app: &mut App) {
        // run this after the Predicted bike gets spawned and has its components synced,
        app.add_systems(
            PreUpdate,
            handle_new_predicted_bike.after(PredictionSet::SpawnHistory),
        );
        app.add_systems(PreUpdate, (handle_new_trail, handle_new_zones).after(MainSet::Receive));
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

/// When a trail is replicated, add the render-related components
fn handle_new_trail(
    mut commands: Commands,
    bike: Query<&ColorComponent, With<BikeMarker>>,
    new_trails: Query<(&Parent, Entity), (With<Trail>, Without<TrailRenderMarker>)>,
) {
    for (parent, entity) in new_trails.iter() {
        if let Ok(color) = bike.get(parent.get()) {
            let trail_color: Color = (color.0.to_linear() * 10.0).into();
            commands.entity(entity)
                .insert((
                        ShapeBundle::default(),
                        TrailRenderMarker,
                        NoFrustumCulling,
                        Stroke::new(trail_color, 1.0),
                ));
        }
    }
}

/// When a zones entity is replicated, add the render-related components
fn handle_new_zones(
    mut commands: Commands,
    bikes: Query<(&BikeMarker, &ColorComponent), With<BikeMarker>>,
    new_zones: Query<(&Parent, Entity), (With<Zones>, Without<ZoneRenderMarker>)>,
) {
    for (parent, entity) in new_zones.iter() {
        if let Ok((bike, color)) = bikes.get(parent.get()) {
            // color values above 1.0 enable bloom
            let c = color.0.to_linear();
            let zone_fill_color: Color = Color::srgba(c.red, c.green, c.blue, 0.15);
            let zone_border_color: Color = (c * 2.0).into();
            let zone_z_order = -(bike.client_id as f32) * 100.0;
            // add the entity that will hold the zone mesh
            commands.entity(entity)
                .insert((
                    ShapeBundle::default(),
                    ZoneRenderMarker,
                    NoFrustumCulling,
                    Fill::color(zone_fill_color),
                    Stroke::new(zone_border_color, 4.0),

                ))
                .insert(
                    GlobalTransform::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        -zone_z_order,
                    )),
                );
        }
    }
}