//! How to draw parry shapes with gizmos. Taken from avian2d
//! https://github.com/Jondolf/avian/blob/main/src%2Fdebug_render%2Fgizmos.rs#L174
#![allow(clippy::unnecessary_cast)]

use avian2d::math::{AsF32, Scalar, Vector};
use avian2d::parry::shape::{SharedShape, TypedShape};
use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::asset::io::memory::Dir;


/// An extension trait for `Gizmos<PhysicsGizmo>`.
pub trait PhysicsGizmoExt {
    /// Draws a line from `a` to `b`.
    fn draw_line(&mut self, a: Vector, b: Vector, color: Color);

    /// Draws lines between a list of points.
    fn draw_line_strip(
        &mut self,
        points: Vec<Vector>,
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        closed: bool,
        color: Color,
    );

    /// Draws a polyline based on the given vertex and index buffers.
    fn draw_polyline(
        &mut self,
        vertices: &[Vector],
        indices: &[[u32; 2]],
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        color: Color,
    );

    /// Draws a [`Collider`] shape.
    fn draw_collider(
        &mut self,
        collider: &Collider,
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        color: Color,
    );
}

impl<'w, 's> PhysicsGizmoExt for Gizmos<'w, 's> {
    /// Draws a line from `a` to `b`.
    fn draw_line(&mut self, a: Vector, b: Vector, color: Color) {
        self.line_2d(a.f32(), b.f32(), color);
    }

    /// Draws lines between a list of points.
    fn draw_line_strip(
        &mut self,
        points: Vec<Vector>,
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        closed: bool,
        color: Color,
    ) {
        let position: Position = position.into();
        let rotation: Rotation = rotation.into();

        let pos = position.f32();
        self.linestrip_2d(points.iter().map(|p| pos + (rotation * p).f32()), color);

        if closed && points.len() > 2 {
            let a = position.0 + rotation * points[0];
            let b = position.0 + rotation * points.last().unwrap();
            self.draw_line(a, b, color);
        }
    }

    /// Draws a polyline based on the given vertex and index buffers.
    fn draw_polyline(
        &mut self,
        vertices: &[Vector],
        indices: &[[u32; 2]],
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        color: Color,
    ) {
        let position: Position = position.into();
        let rotation: Rotation = rotation.into();

        for [i1, i2] in indices {
            let a = position.0 + rotation * vertices[*i1 as usize];
            let b = position.0 + rotation * vertices[*i2 as usize];
            self.draw_line(a, b, color);
        }
    }

    /// Draws a collider shape with a given position and rotation.
    fn draw_collider(
        &mut self,
        collider: &Collider,
        position: impl Into<Position>,
        rotation: impl Into<Rotation>,
        color: Color,
    ) {
        let position: Position = position.into();
        let rotation: Rotation = rotation.into();

        let nalgebra_to_glam =
            |points: &[_]| points.iter().map(|p| Vector::from(*p)).collect::<Vec<_>>();
        match collider.shape_scaled().as_typed_shape() {
            TypedShape::Ball(s) => {
                self.circle(position.extend(0.0).f32(), Dir3::Z, s.radius as f32, color);
            }
            TypedShape::Cuboid(s) => {
                unimplemented!("cannot draw cuboid");
            }
            TypedShape::Capsule(s) => {
                self.draw_line_strip(
                    nalgebra_to_glam(&s.to_polyline(32)),
                    position,
                    rotation,
                    true,
                    color,
                );
            }
            TypedShape::Segment(s) => self.draw_line_strip(
                vec![s.a.into(), s.b.into()],
                position,
                rotation,
                false,
                color,
            ),
            TypedShape::Triangle(s) => self.draw_line_strip(
                vec![s.a.into(), s.b.into(), s.c.into()],
                position,
                rotation,
                true,
                color,
            ),
            TypedShape::TriMesh(s) => {
                for tri in s.triangles() {
                    self.draw_collider(
                        &Collider::from(SharedShape::new(tri)),
                        position,
                        rotation,
                        color,
                    );
                }
            }
            TypedShape::Polyline(s) => self.draw_polyline(
                &nalgebra_to_glam(s.vertices()),
                s.indices(),
                position,
                rotation,
                color,
            ),
            TypedShape::HalfSpace(s) => {
                let basis = Vector::new(-s.normal.y, s.normal.x);
                let a = basis * 10_000.0;
                let b = basis * -10_000.0;
                self.draw_line_strip(vec![a, b], position, rotation, false, color);
            }
            TypedShape::HeightField(s) => {
                for segment in s.segments() {
                    self.draw_collider(
                        &Collider::from(SharedShape::new(segment)),
                        position,
                        rotation,
                        color,
                    );
                }
            }
            TypedShape::Compound(s) => {
                for (sub_pos, shape) in s.shapes() {
                    let pos = Position(position.0 + rotation * Vector::from(sub_pos.translation));
                    let rot = rotation * Rotation::radians(sub_pos.rotation.angle());
                    self.draw_collider(&Collider::from(shape.to_owned()), pos, rot, color);
                }
            }
            TypedShape::ConvexPolygon(s) => {
                self.draw_line_strip(
                    nalgebra_to_glam(s.points()),
                    position,
                    rotation,
                    true,
                    color,
                );
            }
            // ------------
            // Round shapes
            // ------------
            TypedShape::RoundCuboid(s) => {
                self.draw_line_strip(
                    nalgebra_to_glam(&s.to_polyline(32)),
                    position,
                    rotation,
                    true,
                    color,
                );
            }
            TypedShape::RoundTriangle(s) => {
                // Parry doesn't have a method for the rounded outline, so we have to just use a normal triangle
                // (or compute the outline manually based on the border radius)
                self.draw_collider(
                    &Collider::from(SharedShape::new(s.inner_shape)),
                    position,
                    rotation,
                    color,
                );
            }
            TypedShape::RoundConvexPolygon(s) => {
                self.draw_line_strip(
                    nalgebra_to_glam(&s.to_polyline(32)),
                    position,
                    rotation,
                    true,
                    color,
                );
            }
            TypedShape::Custom(_id) =>
                {
                    unimplemented!("cannot draw custom shape");
                }
        }
    }
}