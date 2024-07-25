//! How to render zones

use crate::render::trail::TrailRenderMarker;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::trail::Trail;
use shared::player::{
    bike::ColorComponent,
    zone::{Zone, Zones},
};

pub struct ZoneRenderPlugin;

#[derive(Component)]
pub struct ZoneRenderMarker;

impl Plugin for ZoneRenderPlugin {
    fn build(&self, app: &mut App) {
        // update the trail path after Receive, but before rendering
        app.add_systems(Update, update_zones_path);
    }
}

/// Update the lyon_path (used to filling the zone) of a zone when the zone gets updated
fn update_zones_path(
    mut zones_query: Query<(&Zones, &mut Path), (Changed<Zones>, With<ZoneRenderMarker>)>,
) {
    for (zones, mut path) in zones_query.iter_mut() {
        trace!("update zone: {:?}", zones);
        *path = zones.into();
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct ZoneMaterial {
    #[uniform(100)]
    pub color: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl ZoneMaterial {
    pub fn new(color: Color, texture: Option<Handle<Image>>) -> Self {
        let color = color.to_srgba();
        Self {
            color: Vec4::new(color.red, color.green, color.blue, color.alpha),
            texture,
        }
    }
}

impl Material2d for ZoneMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/zone_material.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/zone_material.wgsl".into()
    // }
}
