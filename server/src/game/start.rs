use bevy::prelude::*;
use lightyear::prelude::{
    server::Replicate, Channel, ChannelKind, NetworkTarget, ReplicateResourceExt,
};
use shared::{
    map::SpawnMap,
    network::protocol::Channel1,
    player::zone::{Zone, ZoneManager},
};

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
    commands.replicate_resource::<ZoneManager, Channel1>(NetworkTarget::All);
    // testing
    // let blue_zone = Zone::new(
    //     vec![
    //         Vec2::new(50.0, 50.0),
    //         Vec2::new(150.0, 50.0),
    //         Vec2::new(150.0, 150.0),
    //         Vec2::new(50.0, 150.0),
    //         Vec2::new(50.0, 50.0),
    //     ],
    //     Color::srgb(0.0, 0.0, 1.0),
    // );

    // let red_zone = Zone::new(
    //     vec![
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(100.0, 0.0),
    //         Vec2::new(100.0, 100.0),
    //         Vec2::new(0.0, 100.0),
    //         Vec2::new(0.0, 0.0),
    //     ],
    //     Color::srgb(1.0, 0.0, 0.0),
    // );

    // // commands.spawn((blue_zone.clone(), Replicate::default()));
    // // commands.spawn((red_zone.clone(), Replicate::default()));

    // let cut_zones = red_zone.cut(&blue_zone);
    // println!("Number of cut zones: {}", cut_zones.len());
    // for (i, zone) in cut_zones.iter().enumerate() {
    //     println!("Zone {}: {:?}", i, zone.points);
    //     commands.spawn((zone.clone(), Replicate::default()));
    // }
}
