//! The 'zone' of the map that is controlled by a player
//! Can be computed with round_convex_decomposition?

use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::physics::util::line_segments_intersect;

#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Zone {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl Zone {
    pub fn new(points: Vec<Vec2>) -> Self {
        Zone {
            points,
            color: Color::WHITE,
        }
    }

    pub fn cut(&self, old: &Zone) -> Vec<Zone> {
        let mut result = Vec::new();
        let mut current_zone = VecDeque::new();
        let mut is_inside = false;

        for i in 0..self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];

            let intersections = Self::find_intersections(&p1, &p2, old);

            if !intersections.is_empty() {
                for intersection in intersections {
                    if is_inside {
                        current_zone.push_back(intersection);
                        result.push(Zone::new(current_zone.drain(..).collect()));
                    } else {
                        current_zone.push_back(p1);
                        current_zone.push_back(intersection);
                    }
                    is_inside = !is_inside;
                }
            }

            if !is_inside {
                current_zone.push_back(p2);
            }
        }

        if !current_zone.is_empty() {
            result.push(Zone::new(current_zone.drain(..).collect()));
        }

        result
    }

    fn find_intersections(p1: &Vec2, p2: &Vec2, zone: &Zone) -> Vec<Vec2> {
        let mut intersections = Vec::new();

        for i in 0..zone.points.len() {
            let q1 = zone.points[i];
            let q2 = zone.points[(i + 1) % zone.points.len()];

            if let Some(intersection) = line_segments_intersect(*p1, *p2, q1, q2) {
                intersections.push(intersection);
            }
        }

        intersections
    }
}
