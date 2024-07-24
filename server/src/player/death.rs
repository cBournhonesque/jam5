use bevy::prelude::*;
use lightyear::prelude::ServerConnectionManager;
use shared::network::message::{KillMessage, KilledByMessage};
use shared::network::protocol::Channel1;
use shared::player::bike::ClientIdMarker;
use shared::player::death::Dead;
use shared::player::scores::{Score, Stats};
use shared::player::trail::Trail;

const KILL_SCORE: u32 = 1000;

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
    mut server: ResMut<ServerConnectionManager>,
    mut commands: Commands,
    mut bikes: Query<(&Children, &ClientIdMarker, &mut Score, &mut Stats), Without<Dead>>,
    mut trails: Query<&mut Trail>,
) {
    let killed = trigger.event().killed;
    let killer = trigger.event().killer;
    if let Some(mut com) = commands.get_entity(killed) {
        com.insert(Dead);
    }
    if let Ok((children, client_id, mut score, mut stats)) = bikes.get_mut(killed) {
        children.into_iter().for_each(|e| {
            // clear the trail
            if let Ok(mut trail) = trails.get_mut(*e) {
                trail.line.clear();
            }
            // TODO: clear the zones?
        });

        *score = Score::default();
        *stats = Stats::default();

        server
            .send_message::<Channel1, _>(client_id.0, &KilledByMessage { killer })
            .expect("could not send message");
    }
    if let Ok((_, client_id, mut score, mut stats)) = bikes.get_mut(killer) {
        score.kill_score += KILL_SCORE;
        stats.kills += 1;
        stats.max_score = stats.max_score.max(score.total());
        server
            .send_message::<Channel1, _>(client_id.0, &KillMessage { killed })
            .expect("could not send message");
    }
}
