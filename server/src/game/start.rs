use bevy::prelude::*;
use lightyear::prelude::server::Replicate;
use shared::{map::SpawnMap, player::zone::Zone};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_start);
    }
}

fn game_start(mut commands: Commands) {
    println!("Game starting!");
    // spawn the map
    commands.trigger(SpawnMap);

    // testing
    let mut old_zone = Zone::new(vec![
        Vec2::new(50.0, 50.0),
        Vec2::new(150.0, 50.0),
        Vec2::new(150.0, 150.0),
        Vec2::new(50.0, 150.0),
    ]);
    old_zone.color = Color::srgb(0.0, 0.0, 1.0);

    let mut new_zone = Zone::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(100.0, 100.0),
        Vec2::new(0.0, 100.0),
    ]);
    new_zone.color = Color::srgb(1.0, 0.0, 0.0);

    let cut_zones = new_zone.cut(&old_zone);
    for zone in cut_zones {
        commands.spawn((zone, Replicate::default()));
    }
}
