use bevy::app::App;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;

const GRID_SIZE: i32 = 100;
const CELL_SIZE: f32 = 50.0;
const GRID_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const LINE_THICKNESS: f32 = 2.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
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