use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use lightyear::prelude::client::*;
use shared::player::bike::BikeMarker;
use crate::assets::{AssetKey, HandleMap};

pub(crate) struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut App) {

        // Plugins
        app.register_type::<HandleMap<ImageKey>>();
        app.init_resource::<HandleMap<ImageKey>>();

        // TODO: draw player
        // TODO: should we worry about transform propagate?
        app.add_systems(PostUpdate, draw_bike);
    }
}


fn draw_bike(
    mut gizmos: Gizmos,
    query: Query<(&Position, &Rotation), (With<BikeMarker>, With<Predicted>)>
) {
    for (pos, rotation) in query.iter() {
        trace!("Drawing bike at {:?}", pos.0);
        gizmos.rounded_rect_2d(
            pos.0,
            rotation.as_radians(),
            Vec2::new(50.0, 10.0),
            Color::WHITE,
        );
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