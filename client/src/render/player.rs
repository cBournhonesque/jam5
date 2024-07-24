use crate::assets::{AssetKey, HandleMap};
use crate::network::BikeSpawned;
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

        app.observe(on_bike_spawned);
        // Draw after TransformPropagate and VisualInterpolation
        app.add_systems(
            PostUpdate,
            (draw_bike_debug, update_bike_position).after(TransformPropagate),
        );
    }
}

#[derive(Component)]
pub struct BikeGraphics {
    followed_entity: Entity,
}

fn on_bike_spawned(
    trigger: Trigger<BikeSpawned>,
    mut commands: Commands,
    image_key: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 6, 6, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    if let Some(texture) = image_key.get(&ImageKey::Moto) {
        commands.spawn((
            BikeGraphics {
                followed_entity: trigger.event().entity,
            },
            SpriteBundle {
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            },
        ));
    }
}

fn update_bike_position(
    q_parents: Query<
        (&Position, &Rotation),
        (
            Or<(Changed<Position>, Changed<Rotation>)>,
            With<BikeMarker>,
            Without<BikeGraphics>,
        ),
    >,
    mut q_bike: Query<(&BikeGraphics, &mut Transform)>,
) {
    for (BikeGraphics { followed_entity }, mut transform) in q_bike.iter_mut() {
        if let Ok((parent_pos, parent_rot)) = q_parents.get(*followed_entity) {
            transform.translation = Vec3::new(parent_pos.0.x, parent_pos.0.y, 100.0);
        }
    }
}

fn draw_bike_debug(
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
    Moto,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Moto,
            asset_server.load_with_settings(
                "images/moto_spritesheet.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into()
    }
}
