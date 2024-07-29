use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};

pub struct MapPlugin;

pub const MAP_SIZE: f32 = 3000.0;

#[derive(Component)]
pub struct MapMarker;

#[derive(Component)]
pub struct MapRadius {
    pub radius: f32,
}

#[derive(Event)]
pub struct SpawnMap;

impl MapPlugin {
    /// Spawn the map when we receive the SpawnMap Trigger.
    pub fn spawn_map(
        _trigger: Trigger<SpawnMap>,
        mut commands: Commands,
        mut global_rng: ResMut<GlobalRng>,
    ) {
        commands.spawn((
            MapRadius { radius: MAP_SIZE },
            MapMarker,
            Name::new("Map"),
            RngComponent::from(&mut global_rng),
        ));
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        //app.observe(Self::spawn_map);
    }
}
