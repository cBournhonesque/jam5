use crate::assets::{AssetKey, HandleMap};
use crate::network::BikeSpawned;
use avian2d::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::reflect::DynamicTypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::sprite::Material2d;
use lightyear::prelude::client::*;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::death::Dead;
use shared::player::trail::Trail;
use shared::player::zone::Zones;

pub(crate) struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.register_type::<HandleMap<ImageKey>>();
        app.register_type::<BikeGraphics>();
        app.init_resource::<HandleMap<ImageKey>>();

        app.observe(on_bike_spawned);
        app.observe(remove_bike_graphics);
        // app.observe(hide_dead_bikes);
        // app.observe(show_respawn_bikes);
        // Draw after TransformPropagate and VisualInterpolation
        app.add_systems(PostUpdate, (update_bike_position).after(TransformPropagate));
    }
}

#[derive(Reflect, Component)]
pub struct BikeGraphics {
    followed_entity: Entity,
}

// NOTE:
// - we don't add the sprite directly on the bike entity because then it would have Transform/GlobalTransform
//   and we don't want the transform to propagate. Or should we just disable Transform Propagation?
// - we don't the sprite entity to be a child of the bike entity because of transform propagation issues as well?
fn on_bike_spawned(
    trigger: Trigger<BikeSpawned>,
    mut commands: Commands,
    image_key: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 6, 6, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    if let Some(texture) = image_key.get(&ImageKey::Moto) {
        commands.spawn((
            BikeGraphics {
                followed_entity: trigger.event().entity,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: trigger.event().color,
                    ..default()
                },
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            },
            Name::from("BikeSprite"),
        ));
    }
}

// fn hide_dead_bikes(
//     trigger: Trigger<OnAdd, Dead>,
//     q_bike: Query<(), Or<(With<Interpolated>, With<Predicted>)>>,
//     mut q_graphics: Query<(&BikeGraphics, &mut Visibility)>,
// ) {
//     if let Ok(()) = q_bike.get(trigger.entity()) {
//         for (q_graphics, mut vis) in q_graphics.iter_mut() {
//             if q_graphics.followed_entity == trigger.entity() {
//                 info!("Hide dead bike");
//                 *vis = Visibility::Hidden;
//             }
//         }
//     }
// }
//
// fn show_respawn_bikes(
//     trigger: Trigger<OnRemove, Dead>,
//     q_bike: Query<(), Or<(With<Interpolated>, With<Predicted>)>>,
//     mut q_graphics: Query<(&BikeGraphics, &mut Visibility)>,
// ) {
//     if let Ok(()) = q_bike.get(trigger.entity()) {
//         for (q_graphics, mut vis) in q_graphics.iter_mut() {
//             if q_graphics.followed_entity == trigger.entity() {
//                 *vis = Visibility::Visible;
//             }
//         }
//     }
// }

const SPRITE_FRAME_COUNT: f32 = 32.0;
const ROTATION_AMOUNT: f32 = 360.0 / SPRITE_FRAME_COUNT;

fn degrees_to_sprite_index(degrees: f32) -> usize {
    ((degrees + 180.0 - (ROTATION_AMOUNT / 2.)) / ROTATION_AMOUNT).floor() as usize
}

fn update_bike_position(
    q_parents: Query<(&Position, &Rotation), (With<BikeMarker>, Without<BikeGraphics>)>,
    mut q_bike: Query<(&BikeGraphics, &mut GlobalTransform, &mut TextureAtlas)>,
) {
    for (BikeGraphics { followed_entity }, mut transform, mut atlas) in q_bike.iter_mut() {
        if let Ok((parent_pos, parent_rot)) = q_parents.get(*followed_entity) {
            *transform = GlobalTransform::from_translation(Vec3::from((parent_pos.0, 100.0)));
            atlas.index = degrees_to_sprite_index(parent_rot.as_degrees());
        }
    }
}

/// If the player disconnects, we want to remove their bike
fn remove_bike_graphics(
    trigger: Trigger<OnRemove, BikeMarker>,
    mut commands: Commands,
    q_graphics: Query<(Entity, &BikeGraphics)>,
) {
    for (entity, graphics) in q_graphics.iter() {
        if graphics.followed_entity == trigger.entity() {
            commands.entity(entity).despawn_recursive();
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
