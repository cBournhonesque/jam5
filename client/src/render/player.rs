use crate::assets::{AssetKey, HandleMap};
use avian2d::prelude::*;
use bevy::prelude::TransformSystem::TransformPropagate;
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use lightyear::prelude::client::*;
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::trail::Trail;
use shared::player::zone::ZoneManager;

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
        app.add_systems(
            PostUpdate,
            (
                draw_bike,
                draw_trail,
                draw_zones.run_if(resource_exists::<ZoneManager>),
            )
                .after(TransformPropagate),
        );
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

fn draw_trail(mut gizmos: Gizmos, query: Query<(&Trail, &ColorComponent), With<BikeMarker>>) {
    for (trail, color) in query.iter() {
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

fn draw_zones(mut gizmos: Gizmos, zone_manager: Res<ZoneManager>) {
    println!("Drawing zones");
    for zone in zone_manager.get_all_zones() {
        println!("Drawing zone: {:?}", zone);
        if zone.points.len() < 3 {
            // Changed from 2 to 3 for closed polygons
            continue;
        }
        for i in 0..zone.points.len() {
            let start = zone.points[i];
            let end = zone.points[(i + 1) % zone.points.len()]; // Use modulo to close the polygon
            gizmos.line_2d(start, end, zone.color);
        }
    }
}

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
