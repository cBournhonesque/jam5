use avian2d::prelude::{Position, Rotation};
use bevy::app::{App, Plugin};
use lightyear::prelude::client::*;

pub mod player;
mod gizmos;

pub(crate) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // add visual interpolation to Position, so that position in Update is interpolated
        // between two FixedUpdate values
        app.add_plugins(VisualInterpolationPlugin::<Position>::default());
        app.add_plugins(VisualInterpolationPlugin::<Rotation>::default());
        app.add_plugins(player::PlayerRenderPlugin);
    }
}
