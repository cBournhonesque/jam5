//! Controls the trails of the bikes
//!
//!

use avian2d::math::Vector;
use avian2d::prelude::*;
use avian2d::parry::shape::Polyline;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Trail {
    line: Polyline,
}

impl Trail {
    pub fn add_point(&mut self, point: Vec<Vector>) {
    }
}