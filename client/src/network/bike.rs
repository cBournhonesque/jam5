//! Module to handle the networking of bikes on the client side

use crate::render::label::EntityLabel;
use crate::render::trail::TrailRenderMarker;
use crate::render::zones::ZoneRenderMarker;
use avian2d::prelude::{Position, RigidBody, Rotation};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_prototype_lyon::prelude::*;
use lightyear::prelude::{client::*, MainSet};
use shared::player::bike::{BikeMarker, ClientIdMarker, ColorComponent};
use shared::player::trail::Trail;
use shared::player::zone::Zones;

use super::BikeSpawned;

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
            handle_new_interpolated_bike.after(InterpolationSet::Interpolate),
        );
        app.add_systems(
            PreUpdate,
            (handle_new_trail, handle_new_zones).after(MainSet::Receive),
        );
        app.add_systems(Update, (add_trail_hierarchy, add_zones_hierarchy));
    }
}

/// When a new trail is added, we go through all bikes to find the parent
fn add_trail_hierarchy(
    mut commands: Commands,
    bike: Query<(Entity, &ClientIdMarker), (With<BikeMarker>, With<Confirmed>)>,
    trails: Query<(Entity, &ClientIdMarker), (With<Trail>, Without<Parent>)>,
) {
    for (trail, trail_client_id) in trails.iter() {
        for (bike, bike_client_id) in bike.iter() {
            if bike_client_id == trail_client_id {
                commands.entity(bike).add_child(trail);
            }
        }
    }
}

/// When a new zones is added, we go through all bikes to find the parent
fn add_zones_hierarchy(
    mut commands: Commands,
    bike: Query<(Entity, &ClientIdMarker), (With<BikeMarker>, With<Confirmed>)>,
    zones: Query<(Entity, &ClientIdMarker), (With<Zones>, Without<Parent>)>,
) {
    for (zones, zone_client_id) in zones.iter() {
        for (bike, bike_client_id) in bike.iter() {
            if bike_client_id == zone_client_id {
                commands.entity(bike).add_child(zones);
            }
        }
    }
}

/// When a interpolated bike gets created, we want to:
/// - add a Player Label
/// - trigger `BikeSpawned` to draw a mesh
fn handle_new_interpolated_bike(
    mut commands: Commands,
    interpolated_bikes: Query<
        (Entity, &ColorComponent, &BikeMarker),
        (With<Interpolated>, Without<EntityLabel>),
    >,
) {
    for (entity, color_component, bike) in interpolated_bikes.iter() {
        commands.entity(entity).insert((EntityLabel {
            text: bike.name.clone(),
            sub_text: "".to_owned(),
            offset: Vec2::new(0.0, 60.0),
            color: color_component.overbright(4.0),
            ..default()
        },));
        commands.trigger(BikeSpawned {
            entity,
            color: color_component.0,
        });
    }
}

/// When a predicted bike gets created, we want to:
/// - add VisualInterpolateStatus component to visually interpolate the bike's Position/Rotation in Update
/// between two FixedUpdate values
/// - add RigidBody::Kinematic component so that the bike is affected by physics
/// - add a Player Label
fn handle_new_predicted_bike(
    mut commands: Commands,
    predicted_bikes: Query<
        (Entity, &ColorComponent, &BikeMarker),
        (With<Predicted>, Without<EntityLabel>),
    >,
) {
    for (entity, color_component, bike) in predicted_bikes.iter() {
        commands.entity(entity).insert((
            VisualInterpolateStatus::<Position>::default(),
            VisualInterpolateStatus::<Rotation>::default(),
            RigidBody::Kinematic,
            EntityLabel {
                text: bike.name.clone(),
                sub_text: "".to_owned(),
                offset: Vec2::new(0.0, 60.0),
                color: color_component.overbright(4.0),
                ..default()
            },
        ));
        commands.trigger(BikeSpawned {
            entity,
            color: color_component.0,
        });
    }
}

/// When a trail is replicated, add the render-related components
fn handle_new_trail(
    mut commands: Commands,
    bike: Query<(&ClientIdMarker, &ColorComponent), With<BikeMarker>>,
    new_trails: Query<(&Parent, Entity), (With<Trail>, Without<TrailRenderMarker>)>,
) {
    for (parent, entity) in new_trails.iter() {
        if let Ok((client_id, color)) = bike.get(parent.get()) {
            let trail_color: Color = color.overbright(10.0);
            let trail_z_order = ((client_id.to_bits() as f32) % 1000.0) / 10.0 + 100.0;
            commands
                .entity(entity)
                .insert((
                    ShapeBundle::default(),
                    TrailRenderMarker,
                    NoFrustumCulling,
                    Stroke::new(trail_color, 1.0),
                ))
                // we insert GlobalTransform separately because ShapeBundle includes GlobalTransform
                .insert(GlobalTransform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    trail_z_order,
                )));
        }
    }
}

/// When a zones entity is replicated, add the render-related components
fn handle_new_zones(
    mut commands: Commands,
    bikes: Query<(&ClientIdMarker, &ColorComponent), With<BikeMarker>>,
    new_zones: Query<(&Parent, Entity), (With<Zones>, Without<ZoneRenderMarker>)>,
) {
    for (parent, entity) in new_zones.iter() {
        if let Ok((client_id, color)) = bikes.get(parent.get()) {
            // color values above 1.0 enable bloom
            let c = color.0.to_linear();
            let zone_fill_color: Color = Color::srgba(c.red, c.green, c.blue, 0.08);
            let zone_border_color: Color = (c * 2.0).into();
            let zone_z_order = ((client_id.to_bits() as f32) % 1000.0) / 10.0;
            // add the entity that will hold the zone mesh
            commands
                .entity(entity)
                .insert((
                    ShapeBundle::default(),
                    ZoneRenderMarker,
                    NoFrustumCulling,
                    Fill::color(zone_fill_color),
                    Stroke::new(zone_border_color, 4.0),
                ))
                // we insert GlobalTransform separately because ShapeBundle includes GlobalTransform
                .insert(GlobalTransform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    -zone_z_order,
                )));
        }
    }
}
