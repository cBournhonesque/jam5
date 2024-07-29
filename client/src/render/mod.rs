use avian2d::prelude::{Position, Rotation};
use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::prelude::ClearColor;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use lightyear::prelude::client::*;

mod chat;
mod diagnostics;
mod egui;
mod kills;
pub mod label;
pub mod map;
mod minimap;
pub mod player;
pub mod trail;
pub mod zones;

pub(crate) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        app.add_plugins(ParticleSystemPlugin);
        // add visual interpolation to Position, so that position in Update is interpolated
        // between two FixedUpdate values
        app.add_plugins((
            VisualInterpolationPlugin::<Position>::default(),
            VisualInterpolationPlugin::<Rotation>::default(),
        ));
        app.add_plugins((
            diagnostics::DiagnosticsPlugin,
            chat::ChatPlugin,
            kills::KillPlugin,
            egui::MyEguiPlugin,
            label::EntityLabelPlugin,
            minimap::MinimapPlugin,
            player::PlayerRenderPlugin,
            trail::TrailRenderPlugin,
            zones::ZoneRenderPlugin,
            map::MapPlugin,
        ));
        app.insert_resource(ClearColor(Color::BLACK));
    }
}
