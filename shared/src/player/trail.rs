use crate::physics::util::line_segments_intersect;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const MIN_POINT_DISTANCE: f32 = 50.0;
const MAX_LINE_POINTS: usize = 200;

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct Trail {
    pub line: Vec<Vec2>,
}

impl Trail {
    pub fn try_add_point(&mut self, point: Vec2) -> Option<Vec<Vec2>> {
        // don't place near the last point
        if let Some(last) = self.line.last() {
            if (*last - point).length() < MIN_POINT_DISTANCE {
                return None;
            }
        }

        // limit the length of the trail
        if self.line.len() > MAX_LINE_POINTS {
            self.line.remove(0);
        }

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
