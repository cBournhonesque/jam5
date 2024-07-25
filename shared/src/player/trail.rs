use crate::physics::util::line_segments_intersect;
use crate::player::bike::ClientIdMarker;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::{GeometryBuilder, Path, PathBuilder};
use bevy_prototype_lyon::shapes;
use lightyear::prelude::{ClientId, DeltaCompression};
use lightyear::shared::replication::delta::Diffable;
use serde::{Deserialize, Serialize};

const MIN_POINT_DISTANCE: f32 = 50.0;

pub const ADD_POINT_INTERVAL: Duration = Duration::from_millis(50);
const MAX_LINE_POINTS: usize = 200;

#[derive(Bundle, Debug)]
pub struct TrailBundle {
    pub trail: Trail,
    pub client: ClientIdMarker,
    pub name: Name,
}

impl TrailBundle {
    pub fn new_at(pos: Vec2, client_id: ClientId) -> Self {
        TrailBundle {
            trail: Trail { line: vec![pos] },
            client: ClientIdMarker(client_id),
            name: Name::from("Trail"),
        }
    }
}

impl Default for TrailBundle {
    fn default() -> Self {
        TrailBundle {
            name: Name::from("Trail"),
            ..default()
        }
    }
}

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Trail>();
    }
}

#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
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
        trace!(
            "Computing trail diff: {diff:?}. Self: {:?} Other: {:?}",
            self.line.len(),
            other.line.len()
        );
        diff
    }

    fn apply_diff(&mut self, delta: &Self::Delta) {
        trace!("Apply trail diff: {delta:?}. Self: {self:?}");
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
    /// Size of the trail
    pub fn len(&self) -> f32 {
        self.line
            .windows(2)
            .map(|pair| (pair[0] - pair[1]).length())
            .sum()
    }

    pub fn try_add_point(&mut self, point: Vec2) -> Option<Vec<Vec2>> {
        if self.line.is_empty() {
            self.line.push(point);
            return None;
        }

        // NOTE: I think it's better to have infinite trails + the min_point_distance makes the trail look
        //  less natural?
        let last_point = *self.line.last().unwrap();

        // if (last_point - point).length() < MIN_POINT_DISTANCE {
        //     return None;
        // }
        //
        // if self.line.len() >= MAX_LINE_POINTS {
        //     self.line.remove(0);
        // }

        if self.line.len() >= 3 {
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

        for (i, window) in self.line.windows(2).enumerate().take(self.line.len() - 2) {
            let line_a = window[0];
            let line_b = window[1];

            if let Some(intersection) = line_segments_intersect(point_a, point_b, line_a, line_b) {
                let mut shape = Vec::new();

                // start from the point after the intersection
                shape.extend_from_slice(&self.line[i + 1..]);

                // add the intersection point to close the loop
                shape.push(intersection);

                return Some(shape);
            }
        }

        None
    }
}
