use std::collections::VecDeque;
use bevy::utils::Duration;
use avian2d::math::Vector;
use avian2d::parry::math::{Isometry, Point};
use avian2d::parry::na::Point2;
use avian2d::parry::query::PointQuery;
use avian2d::parry::shape::{Compound, Polyline, Segment, SharedShape, SupportMap};
use avian2d::parry::transformation::vhacd::{VHACD, VHACDParameters};
use avian2d::parry::transformation::voxelization::FillMode;
use avian2d::prelude::{Collider, ColliderConstructor, CollisionLayers, LinearVelocity, Position};
use crate::physics::util::line_segments_intersect;
use bevy::prelude::*;
use geo::Contains;
use serde::{Deserialize, Serialize};
use crate::player::zone::Zone;

const MIN_POINT_DISTANCE: f32 = 50.0;
const MAX_LINE_POINTS: usize = 200;

pub const ZONE_CORNER_RADIUS: f32 = 8.0;

pub const ADD_POINT_INTERVAL: Duration = Duration::from_millis(100);

// TRAIL:
// - we compute the position, direction at all points. We send a message to the server for the newest points. (maybe send the last few points)
// (so that each message doesn't contain all points. DeltaCompression still doesn't work in lightyear.)
// Maybe the client can also send a message to the server that tells it how many points it has already received?
// Or the server can just send the last few points to the client (in case the client misses any)

// collisions:
// - can use avian2d's collider?
//   - TODO: if we collide with our own line, find the intersection point, detect the interior and create a zone
//   - TODO: if we collide with our existing zone (which is a Shape), detect the interior of the new shape?


/// Message that tells the client to add a new point to the trail
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AddTrailPoint {
    entity: Entity,
    position: Position,
    linear_velocity: LinearVelocity,
    index: usize,
}

#[derive(Bundle, Debug)]
pub struct TrailBundle {
    pub trail: Trail,
    // the collider is the trail itself (it contains a SharedShape::Polyline)
    pub collider: Collider,
    // pub collision_layers: CollisionLayers,
    pub name: Name,
}

impl TrailBundle {
    pub fn new_at(pos: Vector) -> Self {
        Self {
            trail: Trail {
                line: VecDeque::from([pos]),
            },
            collider: Collider::circle(0.0),
            // collision_layers: CollisionLayers::from_bits()
            name: Name::from("Trail"),
        }
    }


    // pub fn add_point(&mut self, point: Vec2) {
    //     if let SharedShape::Polyline
    //     let mut points = self.collider.shape().polyline().unwrap().clone();
    //     points.push(point);
    //     self.collider.set_shape(SharedShape::polyline(points, None));
    // }

}



// we construct a cubic hermic curve from the list of points for rendering
// we can use polyline + parry to detect the interior?

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Trail>();
    }
}


#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct Trail {
    pub line: VecDeque<Vector>,
}

impl From<&Trail> for Polyline {
    fn from(value: &Trail) -> Self {
        let vertices = value.line.iter().map(|v| Point::new(v.x, v.y)).collect();
        Self::new(vertices, None)
    }
}


impl Trail {

    /// Generate a collider from the current trail
    pub fn generate_collider(&self) -> Collider {
        Collider::from(SharedShape::polyline(self.line.iter().map(|v| Point::new(v.x, v.y)).collect(), None))
    }

    pub fn add_point(&mut self, point: Vector) {
        self.line.push_back(point);
    }

    /// Generate a zone from the inside of a polyline that intersects with itself
    /// Copies code from `avian2d::parry::shape::SharedShape::round_convex_decomposition_with_params`
    pub fn generate_zone(&self) -> Zone {
        let mut parts = vec![];
        let polyline = Polyline::from(self);
        let decomp = VHACD::decompose(
            &VHACDParameters {
                fill_mode: FillMode::FloodFill {
                    detect_cavities: false,
                    detect_self_intersections: true,
                },
                // concavity: 0.1,
                // alpha: 1.0,
                // beta: 1.0,
                ..default()
            },
            polyline.vertices(),
            polyline.indices(),
             true);
        for vertices in decomp.compute_exact_convex_hulls(polyline.vertices(), polyline.indices()) {
            if let Some(convex) = SharedShape::convex_polyline(vertices) {
                parts.push((Isometry::identity(), convex));
            }
            // if let Some(convex) = SharedShape::round_convex_polyline(vertices, ZONE_CORNER_RADIUS) {
            //     parts.push((Isometry::identity(), convex));
            // }
        }
        let raw_shapes = parts.into_iter().map(|s| (s.0, s.1)).collect();
        let compound = Compound::new(raw_shapes);
        Zone {
            compound
        }
    }

    // // TODO: 2 options:
    // // - each point is a minimum distance from the previous one
    // // - each point is a fixed amount of time from the previous one (regular sampling)
    // pub fn try_add_point(&mut self, point: Vec2) -> Option<Vec<Vec2>> {
    //     // don't place near the last point
    //     if let Some(last) = self.line.last() {
    //         if (*last - point).length() < MIN_POINT_DISTANCE {
    //             return None;
    //         }
    //     }
    //
    //     // limit the length of the trail
    //     if self.line.len() > MAX_LINE_POINTS {
    //         self.line.remove(0);
    //     }
    //
    //     if self.line.len() >= 3 {
    //         let last_point = *self.line.last().unwrap();
    //         if let Some(shape) = self.detect_intersection(last_point, point) {
    //             self.line = shape;
    //             return Some(self.line.clone());
    //         }
    //     }
    //
    //     self.line.push(point);
    //     None
    // }

    // pub fn detect_intersection(&self, point_a: Vec2, point_b: Vec2) -> Option<Vec<Vec2>> {
    //     if self.line.len() < 3 {
    //         return None;
    //     }
    //     // check all segments except the last one
    //     for i in 0..self.line.len() - 2 {
    //         let line_a = self.line[i];
    //         let line_b = self.line[i + 1];
    //
    //         if let Some(intersection) = line_segments_intersect(point_a, point_b, line_a, line_b) {
    //             // found an intersection, now form the shape
    //             let mut shape = Vec::new();
    //
    //             // add the intersection point
    //             shape.push(intersection);
    //
    //             // add points from the intersection to the end of the trail
    //             shape.extend_from_slice(&self.line[i + 1..]);
    //
    //             // add the intersection point to close off the shape (DO WE NEED THIS?)
    //             shape.push(intersection);
    //
    //             // add the new point
    //             shape.push(point_b);
    //
    //             return Some(shape);
    //         }
    //     }
    //
    //     None
    // }

    /// Cut the part of the trail before the intersection point,
    /// and add an intersection point there to form a loop
    pub fn cut_trail_before_intersection_point(&mut self, bike_point: Vec2) {
        if let Some(cutoff_idx) = self.line.make_contiguous().windows(2).enumerate().find_map(|(i, chunk)| {
            if Segment::from([Point::from(chunk[0]), Point::from(chunk[1])]).project_local_point(
                &Point::from(bike_point), true
            ).is_inside {

            }
            if segment_contains(chunk[0], chunk[1], intersection_point) {
                info!("Found intersection point at index {}", i);
                Some(i)
            } else {
                None
            }
        }) {
            info!("Dropping start of line");
            drop(self.line.drain(..(cutoff_idx+1)));
            self.line.push_front(intersection_point);
        }
    }
}


/// Returns true if the segment [a, b] contains point `point`
fn segment_contains(segment_a: Vector, segment_b: Vector, point: Vector) -> bool {
    info!("Segment contains: {:?} {:?} {:?}", segment_a, segment_b, point);
    geo::Line::new(geo::Point::new(segment_a.x, segment_a.y), geo::Point::new(segment_b.x, segment_b.y))
        .contains(&geo::Point::new(point.x, point.y))
}
