use crate::assets::{AssetKey, HandleMap};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use lightyear::prelude::client::*;
use lightyear::utils::avian2d::linear_velocity;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::trail::Trail;
use shared::player::zone::Zone;

const GRID_SIZE: i32 = 100;
const CELL_SIZE: f32 = 50.0;
const GRID_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const LINE_THICKNESS: f32 = 2.0;

pub(crate) struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.register_type::<HandleMap<ImageKey>>();
        app.init_resource::<HandleMap<ImageKey>>();

        // TODO: draw player
        // TODO: should we worry about transform propagate?
        app.add_systems(Startup, spawn_grid);
        // Draw after TransformPropagate and VisualInterpolation
        app.add_systems(PostUpdate, (draw_bike, draw_trail, draw_zones).after(TransformPropagate));
    }
}

fn draw_bike(
    mut gizmos: Gizmos,
    query: Query<(&Position, &Rotation, &ColorComponent), (With<BikeMarker>, With<Predicted>)>,
) {
    for (pos, rotation, color) in query.iter() {
        trace!("Drawing bike at {:?}", pos.0);
        gizmos.rounded_rect_2d(
            pos.0,
            rotation.as_radians(),
            Vec2::new(50.0, 10.0),
            color.0,
        );
    }
}

fn draw_trail(
    mut gizmos: Gizmos,
    bike_query: Query<&ColorComponent>,
    trail_query: Query<(&Parent, &Trail)>
) {
    for (parent, trail) in trail_query.iter() {
        if let Ok(color) = bike_query.get(parent.get()) {
            let trail_color = Color::Hsva(Hsva {
                saturation: 0.4,
                ..Hsva::from(color.0)
            });
            if trail.line.len() < 2 {
                continue;
            }
            for i in 0..trail.line.len() - 1 {
                let start = trail.line[i];
                let end = trail.line[i + 1];
                gizmos.line_2d(start, end, trail_color);
            }
        }
    }
}

fn draw_zones(
    mut gizmos: Gizmos,
    bike_query: Query<&ColorComponent>,
    zone_query: Query<(&Parent, &Zone)>
) {
    for (parent, zone) in zone_query.iter() {
        if let Ok(color) = bike_query.get(parent.get()) {
            let zone_color = Color::Hsva(Hsva {
                saturation: 0.2,
                ..Hsva::from(color.0)
            });
            zone.compound.shapes().iter().for_each(|(_, shape)| {
                let polygon = shape.as_convex_polygon().unwrap();
                polygon.points().chunks_exact(2).for_each(|pair| {
                    gizmos.line_2d(Vec2::from(pair[0]), Vec2::from(pair[1]), zone_color);
                });

                // ROUND POLYLINES
                // let polygon = shape.as_round_convex_polygon().unwrap();
                // let line = polygon.to_polyline(100);
                // line.chunks_exact(2).for_each(|pair| {
                //     gizmos.line_2d(Vec2::from(pair[0]), Vec2::from(pair[1]), zone_color);
                // });
            })
        }
    }
}

// fn draw_zones(mut gizmos: Gizmos, query: Query<&Zone>) {
//     for zone in query.iter() {
//         println!("Drawing zone: {:?}", zone);
//         if zone.points.len() < 2 {
//             continue;
//         }
//         for i in 0..zone.points.len() - 1 {
//             let start = zone.points[i];
//             let end = zone.points[i + 1];
//             gizmos.line_2d(start, end, zone.color);
//         }
//     }
// }

fn spawn_grid(mut commands: Commands) {
    let total_width = GRID_SIZE as f32 * CELL_SIZE;
    let total_height = GRID_SIZE as f32 * CELL_SIZE;
    let offset = Vec3::new(-total_width / 2.0, -total_height / 2.0, 0.0);

    // Spawn horizontal lines
    for i in 0..=GRID_SIZE {
        let position = Vec3::new(total_width / 2.0, i as f32 * CELL_SIZE, 0.0) + offset;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: GRID_COLOR,
                custom_size: Some(Vec2::new(total_width, LINE_THICKNESS)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        });
    }

    // Spawn vertical lines
    for i in 0..=GRID_SIZE {
        let position = Vec3::new(i as f32 * CELL_SIZE, total_height / 2.0, 0.0) + offset;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: GRID_COLOR,
                custom_size: Some(Vec2::new(LINE_THICKNESS, total_height)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
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
