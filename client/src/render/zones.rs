//! How to render zones

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use bevy_prototype_lyon::prelude::Path;
use lightyear::prelude::MainSet;
use shared::player::{
    bike::ColorComponent,
    zone::{Zone, Zones},
};

pub struct ZoneRenderPlugin;

#[derive(Component)]
pub struct ZoneRenderMarker;

impl Plugin for ZoneRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (update_zone_path.after(MainSet::Receive), draw_zone_outlines),
        );
    }
}

/// Update the path of a zone when the zone gets updated
fn update_zone_path(
    zone_query: Query<&Zones, Or<(Added<Zones>, Changed<Zones>)>>,
    mut zone_render_query: Query<(&Parent, &mut Path), With<ZoneRenderMarker>>,
) {
    for (parent, mut path) in zone_render_query.iter_mut() {
        if let Ok(zones) = zone_query.get(parent.get()) {
            *path = zones.into();
        }
    }
}

fn draw_zone_outlines(mut gizmos: Gizmos, query: Query<(&Zones, &ColorComponent)>) {
    for (zones, color) in query.iter() {
        let color = Color::Hsva(Hsva {
            saturation: 0.4,
            ..Hsva::from(color.0)
        });
        for zone in zones.zones.iter() {
            // draw exterior outline
            for i in 0..zone.exterior.len() - 1 {
                let start = zone.exterior[i];
                let end = zone.exterior[i + 1];
                gizmos.line_2d(start, end, color);
            }
            // close the exterior polygon
            if !zone.exterior.is_empty() {
                let start = zone.exterior[zone.exterior.len() - 1];
                let end = zone.exterior[0];
                gizmos.line_2d(start, end, color);
            }

            // draw interior holes
            for interior in zone.interiors.iter() {
                for i in 0..interior.len() - 1 {
                    let start = interior[i];
                    let end = interior[i + 1];
                    gizmos.line_2d(start, end, color);
                }
                // close the interior polygon
                if !interior.is_empty() {
                    let start = interior[interior.len() - 1];
                    let end = interior[0];
                    gizmos.line_2d(start, end, color);
                }
            }
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

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/zone_material.wgsl".into()
    // }
}
