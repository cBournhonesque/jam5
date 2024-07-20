use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::transform::TransformSystem;
use lightyear::prelude::client::*;
use lightyear::prelude::{TickManager};

use shared::network::protocol::prelude::*;
use crate::assets::{AssetKey, HandleMap};

pub(crate) struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut App) {

        // Plugins
        app.register_type::<HandleMap<ImageKey>>();
        app.init_resource::<HandleMap<ImageKey>>();

        // Draw the players after visual interpolation is computed
        app.add_systems(PostUpdate, draw_player
            .after(TransformSystem::TransformPropagate)
            .after(InterpolationSet::VisualInterpolation)
        );
    }
}

/// System that draws the boxed of the player positions.
/// The components should be replicated from the server to the client
pub(crate) fn draw_player(
    mut gizmos: Gizmos,
    tails: Query<&TailPoints, Without<Confirmed>>,
    interp_snake: Query<&TailPoints, With<Interpolated>>,
    predicted_snake: Query<&TailPoints, With<Predicted>>,
    tick: Res<TickManager>,
) {
    let tick = tick.tick();
    for points in interp_snake.iter() {
        // info!(?tick, front = ?points.front(), "interp snake");
    }
    for points in predicted_snake.iter() {
        // info!(?tick, front = ?points.front(), "predicted snake");
    }
    for points in tails.iter() {
        // draw the head
        gizmos.rect_2d(
            points.front().0,
            0.0,
            Vec2::ONE * 10.0,
            Color::BLUE
        );
        points.pairs_front_to_back().for_each(|(start, end)| {
            gizmos.line_2d(start.0, end.0, Color::BLUE);
            if start.0.x != end.0.x && start.0.y != end.0.y {
                info!("DIAGONAL");
            }
        });
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