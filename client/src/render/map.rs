use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::NoFrustumCulling,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::{
    draw::Fill,
    entity::{Path, ShapeBundle},
    path::PathBuilder,
    prelude::GeometryBuilder,
    shapes,
};
use shared::map::MAP_SIZE;
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct MapMaterial {
    #[uniform(100)]
    pub color: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl MapMaterial {
    pub fn new(color: Color, texture: Option<Handle<Image>>) -> Self {
        let color = color.to_srgba();
        Self {
            color: Vec4::new(color.red, color.green, color.blue, color.alpha),
            texture,
        }
    }
}

impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/map_material.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/map_material.wgsl".into()
    // }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<MapMaterial>::default());
        app.add_systems(Startup, setup_map);
    }
}

fn setup_map(
    mut materials: ResMut<Assets<MapMaterial>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let radius = MAP_SIZE;

    // isometric ratio (approximately 0.866, which is sqrt(3)/2)
    let iso_ratio = 0.866;

    let shape = shapes::Ellipse {
        radii: Vec2::new(radius, radius * iso_ratio),
        center: Vec2::ZERO,
    };

    info!("Creating map with radius: {}", radius);
    let map_color = Color::srgb(0.15, 0.1, 0.3);
    commands.spawn((
        // ShapeBundle {
        //     path: GeometryBuilder::build_as(&shape),
        //     ..Default::default()
        // },
        Path::from(GeometryBuilder::build_as(&shape)),
        MaterialMesh2dBundle {
            material: materials.add(MapMaterial::new(map_color, None)),
            ..default()
        },
        NoFrustumCulling,
        Fill::color(map_color),
    ));
}
