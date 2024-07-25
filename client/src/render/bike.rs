use crate::assets::{AssetKey, HandleMap};
use crate::network::BikeSpawned;
use avian2d::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::reflect::DynamicTypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::sprite::Material2d;
use bevy_hanabi::prelude::*;
use bevy_hanabi::{EffectAsset, ParticleEffect, ParticleEffectBundle};
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
    mut effects: ResMut<Assets<EffectAsset>>,
    image_key: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 6, 6, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    let color = trigger.event().color;
    if let Some(texture) = image_key.get(&ImageKey::Moto) {
        commands
            .spawn((
                BikeGraphics {
                    followed_entity: trigger.event().entity,
                },
                SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    texture: texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_handle,
                    index: 0,
                },
                Name::from("BikeSprite"),
            ))
            .insert(ParticleEffectBundle {
                effect: ParticleEffect::new(create_bike_particle_effect(color, &mut effects))
                    .with_z_layer_2d(Some(1.0)),
                transform: Transform {
                    translation: Vec3::new(100.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            });
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

const ROTATION_AMOUNT: f32 = 360.0 / 32.0;

fn degrees_to_sprite_index(degrees: f32) -> usize {
    ((degrees + 180.0) / ROTATION_AMOUNT).floor() as usize
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

pub fn create_bike_particle_effect(
    color: Color,
    effects: &mut Assets<EffectAsset>,
) -> Handle<EffectAsset> {
    let color = color.to_linear();
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(color.red, color.green, color.blue, 1.0));
    gradient.add_key(1.0, Vec4::new(color.red, color.green, color.blue, 0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(25.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(10.0).expr(),
    };

    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut module = writer.finish();

    let spawner = Spawner::rate(50.0.into());
    effects.add(
        EffectAsset::new(vec![4096], spawner, module)
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(5.0)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    )
}
