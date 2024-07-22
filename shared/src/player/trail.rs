use bevy::utils::Duration;
use crate::physics::util::line_segments_intersect;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{GeometryBuilder, Path, PathBuilder};
use bevy_prototype_lyon::shapes;
use lightyear::prelude::DeltaCompression;
use lightyear::shared::replication::delta::Diffable;
use serde::{Deserialize, Serialize};

const MIN_POINT_DISTANCE: f32 = 50.0;

pub const ADD_POINT_INTERVAL: Duration = Duration::from_millis(50);
const MAX_LINE_POINTS: usize = 200;

#[derive(Bundle, Debug)]
pub struct TrailBundle {
    pub trail: Trail,
    pub name: Name,
}

impl TrailBundle {
    pub fn new_at(pos: Vec2) -> Self {
        TrailBundle {
            trail: Trail {
                line: vec![pos],
            },
            name: Name::from("Trail"),
        }
    }
}

impl Default for TrailBundle {
    fn default() -> Self {
        TrailBundle {
            trail: Trail::default(),
            name: Name::from("Trail"),
        }
    }
}

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct Trail {
    pub line: Vec<Vec2>,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct TrailDiff {
    pub line_diff: Vec<Vec2>,
    /// If true, the `line_diff` is a new line instead of being an extension
    /// of the previous one
    pub new_line: bool,
}

/// Note that DeltaCompression currently doesn't work if the replicated entity has multiple
/// components that are updated at different times (because we keep track of the entity's last_ack per entity, not per component)
/// so we will need to store the trail in a separate entity.
impl Diffable for Trail {
    type Delta = TrailDiff;

    fn base_value() -> Self {
        Trail::default()
    }

    fn diff(&self, other: &Self) -> Self::Delta {
        // other is a new trail, so we need to send the whole thing
        let diff = if other.line.len() < self.line.len() {
            TrailDiff {
                line_diff: other.line.clone(),
                new_line: true,
            }
        } else {
            // else only send the new points
            TrailDiff {
                line_diff: other.line[self.line.len()..].to_vec(),
                new_line: false,
            }
        };
        info!("Computing trail diff: {diff:?}. Self: {:?} Other: {:?}", self.line.len(), other.line.len());
        diff
    }

    fn apply_diff(&mut self, delta: &Self::Delta) {
        info!("Apply trail diff: {delta:?}. Self: {self:?}");
        if delta.new_line {
            self.line = delta.line_diff.clone();
        } else {
            self.line.extend(delta.line_diff.iter().cloned());
        }
    }
}


impl From<&Trail> for Path {
    fn from(value: &Trail) -> Self {
        let mut path = PathBuilder::new();
        if value.line.len() < 2 {
            return path.build();
        }
        path.move_to(value.line[0]);
        for point in value.line.iter().skip(1) {
            path.line_to(*point);
        }
        path.build()
    }
}

impl Trail {
    pub fn try_add_point(&mut self, point: Vec2) -> Option<Vec<Vec2>> {
        // // don't place near the last point
        // if let Some(last) = self.line.last() {
        //     if (*last - point).length() < MIN_POINT_DISTANCE {
        //         return None;
        //     }
        // }
        //
        // // limit the length of the trail
        // if self.line.len() > MAX_LINE_POINTS {
        //     self.line.remove(0);
        // }

        if self.line.len() >= 3 {
            let last_point = *self.line.last().unwrap();
            if let Some(shape) = self.detect_intersection(last_point, point) {
                self.line = shape;
                return Some(self.line.clone());
            }
        }

        self.line.push(point);
        None
    }

    pub fn detect_intersection(&self, point_a: Vec2, point_b: Vec2) -> Option<Vec<Vec2>> {
        if self.line.len() < 3 {
            return None;
        }
        // check all segments except the last one
        for i in 0..self.line.len() - 2 {
            let line_a = self.line[i];
            let line_b = self.line[i + 1];

            if let Some(intersection) = line_segments_intersect(point_a, point_b, line_a, line_b) {
                // found an intersection, now form the shape
                let mut shape = Vec::new();

                // add the intersection point
                shape.push(intersection);

                // add points from the intersection to the end of the trail
                shape.extend_from_slice(&self.line[i + 1..]);

                // add the intersection point to close off the shape (DO WE NEED THIS?)
                shape.push(intersection);

                // add the new point
                shape.push(point_b);

                return Some(shape);
            }
        }

        None
    }
}
