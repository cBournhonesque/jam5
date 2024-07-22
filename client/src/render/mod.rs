use avian2d::prelude::{Position, Rotation};
use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy_prototype_lyon::prelude::ShapePlugin;
use lightyear::prelude::client::*;

pub mod player;
pub mod trail;
pub mod zones;
mod grid;
mod diagnostics;

pub(crate) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        // add visual interpolation to Position, so that position in Update is interpolated
        // between two FixedUpdate values
        app.add_plugins((
                            VisualInterpolationPlugin::<Position>::default(),
                            VisualInterpolationPlugin::<Rotation>::default()
                        ));
        app.add_plugins( (
            diagnostics::DiagnosticsPlugin,
            // grid::GridPlugin;
            player::PlayerRenderPlugin,
            trail::TrailRenderPlugin,
            zones::ZoneRenderPlugin,
        ));
    }
}
