use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::ServerConnectionManager;
use shared::network::message::{KillMessage, KilledByMessage};
use shared::network::protocol::Channel1;
use shared::player::bike::{BikeMarker, ClientIdMarker};
use shared::player::death::{Dead, DeathTimer, DEATH_TIMER};
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
        app.add_systems(Update, respawn_player);
        app.observe(kill_player);
    }
}

/// Tick death timers and respawn players
fn respawn_player(
    mut commands: Commands,
    time: Res<Time>,
    mut dead: Query<(Entity, &mut BikeMarker, &mut DeathTimer), With<Dead>>,
) {
    for (entity, mut bike, mut timer) in dead.iter_mut() {
        timer.respawn_timer.tick(time.delta());
        if timer.respawn_timer.finished() {
            bike.spawn_time = time.elapsed();
            commands
                .entity(entity)
                .insert((
                    Position::default(),
                    Rotation::default(),
                    LinearVelocity::default(),
                ))
                .remove::<Dead>()
                .remove::<DeathTimer>();
        }
    }
}

/// Observer that handles player_kill_events to put bikes in Dead state
fn kill_player(
    trigger: Trigger<PlayerKillEvent>,
    time: Res<Time>,
    mut server: ResMut<ServerConnectionManager>,
    mut commands: Commands,
    mut bikes: Query<
        (
            &Children,
            &BikeMarker,
            &ClientIdMarker,
            &mut Position,
            &mut Score,
            &mut Stats,
        ),
        Without<Dead>,
    >,
    mut trails: Query<&mut Trail>,
) {
    let killed = trigger.event().killed;
    let killer = trigger.event().killer;
    if let Some(mut com) = commands.get_entity(killed) {
        com.insert(Dead);
        com.insert(DeathTimer {
            respawn_timer: Timer::new(DEATH_TIMER, TimerMode::Once),
        });
    }
    if let Ok((children, bike, client_id, mut position, mut score, mut stats)) =
        bikes.get_mut(killed)
    {
        // we send dead bikes to narnia
        *position = Position::new(Vec2::new(100000.0, 1000000.0));

        children.into_iter().for_each(|e| {
            // clear the trail
            if let Ok(mut trail) = trails.get_mut(*e) {
                trail.line.clear();
            }
            // TODO: clear the zones when a player is killed?
        });

        stats.time_lived_secs = (time.elapsed() - bike.spawn_time).as_secs() as u32;

        server
            .send_message::<Channel1, _>(
                client_id.0,
                &KilledByMessage {
                    killer,
                    stats: stats.clone(),
                },
            )
            .expect("could not send message");

        *score = Score::default();
        *stats = Stats::default();
    }
    if let Ok((_, _, client_id, _, mut score, mut stats)) = bikes.get_mut(killer) {
        score.kill_score += KILL_SCORE;
        stats.kills += 1;
        stats.max_score = stats.max_score.max(score.total());
        server
            .send_message::<Channel1, _>(client_id.0, &KillMessage { killed })
            .expect("could not send message");
    }
}
