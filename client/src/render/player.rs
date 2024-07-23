use crate::assets::{AssetKey, HandleMap};
use avian2d::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::sprite::Material2d;
use lightyear::prelude::client::*;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::trail::Trail;
use shared::player::zone::Zones;

pub(crate) struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.register_type::<HandleMap<ImageKey>>();
        app.init_resource::<HandleMap<ImageKey>>();

        // Draw after TransformPropagate and VisualInterpolation
        app.add_systems(PostUpdate, (draw_bike,).after(TransformPropagate));
    }
}

fn draw_bike(
    mut gizmos: Gizmos,
    query: Query<(&Position, &Rotation, &ColorComponent), (With<BikeMarker>, With<Predicted>)>,
) {
    for (pos, rotation, color) in query.iter() {
        trace!("Drawing bike at {:?}", pos.0);
        gizmos.rounded_rect_2d(pos.0, rotation.as_radians(), Vec2::new(50.0, 10.0), color.0);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into()
    }
}
