//! How to render zones

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::zone::{Zone, Zones};

pub struct ZoneRenderPlugin;

#[derive(Component)]
pub struct ZoneRenderMarker;

impl Plugin for ZoneRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_zone_path.after(MainSet::Receive));
        app.add_plugins(Material2dPlugin::<ZoneMaterial>::default());
    }
}

/// Update the path of a zone when the zone gets updated
fn update_zone_path(
    zone_query: Query<&Zones, Changed<Zones>>,
    mut zone_render_query: Query<(&Parent, &mut Path), With<ZoneRenderMarker>>,
) {
    for (parent, mut path) in zone_render_query.iter_mut() {
        if let Ok(zones) = zone_query.get(parent.get()) {
            *path = zones.into();
        }
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
}
