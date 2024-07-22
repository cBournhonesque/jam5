use avian2d::math::Vector;
use avian2d::parry::shape::Polyline;
use avian2d::parry::transformation::vhacd::VHACDParameters;
use avian2d::prelude::*;
use avian2d::prelude::contact_query::time_of_impact;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use bevy::time::common_conditions::on_timer;
use lightyear::prelude::server::Replicate;
use shared::physics::FixedSet;
use shared::player::trail::ADD_POINT_INTERVAL;
use shared::player::{bike::BikeMarker, trail::Trail, zone::Zone};

// TODO: this should depend on speed no?
pub const CONTACT_DISTANCE: f32 = 0.1;


pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            // after we have advanced objects with physics, maybe add a point
            (add_points_to_trail.run_if(on_timer(ADD_POINT_INTERVAL)), detect_self_loop_contact)
                .chain()
                .after(FixedSet::Physics)
        );
        // app.add_systems(FixedUpdate, mark_trail_system);
    }
}

// fn handle_collisions(
//
//
// )

#[derive(Event, Debug)]
pub(crate) struct SelfLoopContactEvent {
    // trail_entity: Entity,
    pub(crate) contact_point: Vector,
}

/// Detect if a bike will collide with its own trail
fn detect_self_loop_contact(
    mut commands: Commands,
    bike: Query<(&Position, &Rotation, &LinearVelocity, &Collider), With<BikeMarker>>,
    trail: Query<(Entity, &Parent, &Collider), With<Trail>>,
) {
    for (trail_entity, parent, trail_collider) in trail.iter() {
        if let Ok((pos, rot, vel, bike_collider)) = bike.get(parent.get()) {
            if let Ok(impact) = time_of_impact(
                bike_collider,
                // avoid self collisions by advancing by epsilon
                Position(pos.0 + vel.0 * 0.01),
                *rot,
                *vel,
                trail_collider,
                Position::default(),
                Rotation::default(),
                LinearVelocity::default(),
                CONTACT_DISTANCE
            ) {
                if let Some(impact) = impact {
                    info!("Impact of a bike with its trail ({trail_entity:?}) at {:?}. Head is at {:?}", impact.point2, impact.point1);
                    commands.trigger_targets(SelfLoopContactEvent {
                        // trail_entity,
                        contact_point: impact.point2,
                    }, trail_entity);
                }
            } else {
                error!("Could not find time of impact")
            }
        }
    }
}



// /// Handle situations where the trail forms a loop
// /// Perform a ray-cast query to check
// fn handle_trail_loops(
//     mut commands: Commands,
//     spatial_query: SpatialQuery,
//     bikes: Query<(&Position, &LinearVelocity)>,
//     tails: Query<(Entity, &Parent, &Trail)>) {
//     for (entity, parent, trail) in tails.iter() {
//         if let Ok((position, velocity)) = bikes.get(parent.get()) {
//             spatial_query.ray_hits(
//                 position.0,
//                 velocity.0.into(),
//                 CONTACT_DISTANCE,
//                 3,
//                 false,
//                 // TODO: maybe exclude trails and zones from other players?
//                 SpatialQueryFilter::default(),
//             )
//                 // we only care about self-loop hits
//                 .filter(|result| result.entity == entity)
//                 .find_map(|result| {
//                     // we found a self-loop! Close the loop and spawn a new zone?
//                     commands.trigger()
//
//
//                 });
//             })
//         }
//     }
// }

/// Add points to each trail, and update the colliders accordingly
fn add_points_to_trail(
    mut bike_query: Query<(&Position, &Rotation, &LinearVelocity)>,
    mut trail_query: Query<(&Parent, &mut Trail, &mut Collider)>,
) {
    for (parent, mut trail, mut collider) in trail_query.iter_mut() {
        if let Ok(bike_position) = bike_query.get(parent.get()) {
            let point = bike_position.0;
            trail.add_point(point.0);
            *collider = trail.generate_collider();
        }
    }
}

// fn mark_trail_system(
//     mut commands: Commands,
//     mut query: Query<(&Position, &mut Trail), With<BikeMarker>>,
// ) {
//     for (position, mut trail) in query.iter_mut() {
//         let point = position.0;
//         if let Some(shape) = trail.try_add_point(point) {
//             commands.spawn((Zone::new(shape), Replicate::default()));
//             // TODO: spawn the shape https://docs.rs/parry2d/latest/parry2d/shape/struct.SharedShape.html#method.round_convex_decomposition_with_params
//             // TODO: temporarily disable the trail?
//             // TODO: total up surface area and increment score based on that?
//         }
//     }
// }
