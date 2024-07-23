use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
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
    }
}
