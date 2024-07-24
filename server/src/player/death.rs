
use bevy::prelude::*;
use shared::player::bike::BikeMarker;
use shared::player::death::Dead;
use shared::player::trail::Trail;

pub struct DeathPlugin;


#[derive(Event)]
pub struct PlayerKillEvent {
    pub killer: Entity,
    pub killed: Entity,
}


impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.observe(kill_player);
    }
}


/// Observer that handles player_kill_events to put bikes in Dead state
fn kill_player(
    trigger: Trigger<PlayerKillEvent>,
    mut commands: Commands,
    bikes: Query<&Children, (With<BikeMarker>, Without<Dead>)>,
    mut trails: Query<&mut Trail>,
) {
    let killed = trigger.event().killed;
    if let Some(mut com) = commands.get_entity(killed) {
        com.insert(Dead);
    }
    if let Ok(children) = bikes.get(killed) {
        children.into_iter().for_each(|e| {
            // clear the trail
            if let Ok(mut trail) = trails.get_mut(*e) {
                trail.line.clear();
            }
            // TODO: clear the zones?
        })
    }
    // TODO: send a message and increase score
}