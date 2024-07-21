use avian2d::{
    parry::{na::Point2, shape::Polyline},
    position::Position,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, mark_trail_system);
    }
}

#[derive(Component)]
pub struct Trail {
    first_point: Option<Point2<f32>>,
    pub line: Option<Polyline>,
}

impl Default for Trail {
    fn default() -> Self {
        Self {
            first_point: None,
            line: None,
        }
    }
}

impl Trail {
    pub fn add_point(&mut self, point: Vec2) {
        let new_point = Point2::new(point.x, point.y);

        match (&self.first_point, &mut self.line) {
            // first point
            (None, _) => {
                println!("first point");
                self.first_point = Some(new_point);
            }
            // second point, create a line
            (Some(first), None) => {
                println!("second point");
                let points = vec![*first, new_point];
                let indices = vec![[0, 1]];
                self.line = Some(Polyline::new(points, Some(indices)));
                self.first_point = None;
            }
            // add to existing line
            (_, Some(line)) => {
                println!("adding point to line");
                let mut points = line.vertices().to_vec();
                points.push(new_point);
                let indices = Self::compute_indices(&points);
                *line = Polyline::new(points, Some(indices));
            }
        }
    }

    fn compute_indices(points: &[Point2<f32>]) -> Vec<[u32; 2]> {
        (0..points.len().saturating_sub(1))
            .map(|i| [i as u32, (i + 1) as u32])
            .collect()
    }
}

fn mark_trail_system(mut query: Query<(&Position, &mut Trail)>) {
    println!("marking trail");
    for (position, mut trail) in query.iter_mut() {
        println!("adding point to trail");
        trail.add_point(position.0);
    }
}
