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
use bevy_particle_systems::EmitterShape::Line;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing, VelocityModifier,
};
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
    bike: Query<&ColorComponent, With<BikeMarker>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 6, 6, None, None);
    let texture_atlas_handle = texture_atlas_layouts.add(layout);
    if let Some(texture) = image_key.get(&ImageKey::Moto) {
        if let Ok(color) = bike.get(trigger.event().entity) {
            commands
                .spawn((
                    BikeGraphics {
                        followed_entity: trigger.event().entity,
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            color: trigger.event().color,
                            custom_size: Some(Vec2::new(128.0, 128.0)),
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ParticleSystemBundle {
                            particle_system: ParticleSystem {
                                lifetime: JitteredValue::jittered(0.5, -0.20..0.2),
                                spawn_rate_per_second: 50.0.into(),
                                max_particles: 200,
                                initial_speed: JitteredValue::jittered(500.0, -200.0..200.0),
                                initial_scale: JitteredValue::jittered(3.0, -2.0..1.0),
                                scale: (1.0..0.0).into(),
                                velocity_modifiers: vec![VelocityModifier::Drag(0.005.into())],
                                color: ColorOverTime::Gradient(Curve::new(vec![
                                    CurvePoint::new(color.overbright(5.0), 0.0),
                                    CurvePoint::new(color.overbright(1.0), 1.0),
                                ])),
                                looping: true,
                                ..default()
                            },
                            ..default()
                        },
                        Playing,
                        Name::from("TrailParticles"),
                    ));
                });
        }
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

/// Update the bike sprite graphics and the trail particles when the bike moves
fn update_bike_position(
    mut q_particles: Query<
        (&Parent, &mut GlobalTransform, &mut ParticleSystem),
        (With<Playing>, Without<BikeGraphics>),
    >,
    q_parents: Query<
        (&Position, &Rotation, &LinearVelocity),
        (With<BikeMarker>, Without<BikeGraphics>),
    >,
    mut q_bike: Query<(&BikeGraphics, &mut GlobalTransform, &mut TextureAtlas)>,
) {
    for (parent, mut particle_transform, mut particles) in q_particles.iter_mut() {
        if let Ok((BikeGraphics { followed_entity }, mut transform, mut atlas)) =
            q_bike.get_mut(parent.get())
        {
            if let Ok((parent_pos, parent_rot, velocity)) = q_parents.get(*followed_entity) {
                // let particle_angle_jitter = std::f32::consts::PI / 12.0;
                let particle_angle = parent_rot.as_radians();
                particles.emitter_shape = EmitterShape::CircleSegment(CircleSegment {
                    opening_angle: std::f32::consts::PI * 0.15,
                    direction_angle: particle_angle + std::f32::consts::PI,
                    ..default()
                });
                particles.spawn_rate_per_second = (velocity.0.length() * 0.1).into();
                // particles.emitter_shape = EmitterShape::line::<std::ops::Range<f32>>(
                //     10.0,
                //     ((particle_angle - particle_angle_jitter)
                //         ..(particle_angle + particle_angle_jitter))
                //         .into(),
                // );
                // we put particles slightly in the back
                let particle_pos = parent_pos.0 - Vec2::new(parent_rot.cos, parent_rot.sin) * 30.0;
                *particle_transform =
                    GlobalTransform::from_translation(Vec3::from((particle_pos, 100.0)));
                *transform = GlobalTransform::from_translation(Vec3::from((parent_pos.0, 100.0)));
                atlas.index = degrees_to_sprite_index(parent_rot.as_degrees());
            }
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
