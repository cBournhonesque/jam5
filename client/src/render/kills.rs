use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use bevy_egui::{egui, EguiContexts};
use lightyear::prelude::client::*;
use rand::prelude::SliceRandom;
use shared::network::message::{KillMessage, KilledByMessage};
use shared::player::bike::{BikeMarker, ClientIdMarker};
use shared::player::death::DEATH_TIMER;
use shared::player::scores::{Score, Stats};
use std::time::Duration;

const KILL_MESSAGE_DURATION: Duration = Duration::from_secs(3);

pub struct KillPlugin;

impl Plugin for KillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KillMessages>();
        app.insert_resource(KilledByMessageRes {
            message: "".to_string(),
            stats: Stats::default(),
            timer: None,
        });
        app.add_systems(Update, (handle_kill_message, handle_killed_by_message));
    }
}

#[derive(Resource, Default)]
pub struct KillMessages {
    pub(crate) messages: Vec<(String, Timer)>,
}

#[derive(Resource, Default)]
pub struct KilledByMessageRes {
    pub message: String,
    pub(crate) stats: Stats,
    pub(crate) timer: Option<Timer>,
}

fn handle_killed_by_message(
    time: Res<Time>,
    players: Query<&BikeMarker, With<Confirmed>>,
    mut res: ResMut<KilledByMessageRes>,
    mut messages: ResMut<Events<MessageEvent<KilledByMessage>>>,
) {
    for message in messages.drain() {
        let name = players
            .get(message.message.killer)
            .map_or("Someone".to_string(), |bike| bike.name.clone());
        res.message = format!("Killed by {}", name);
        res.stats = message.message.stats;
        res.timer = Some(Timer::new(DEATH_TIMER, TimerMode::Once));
    }
    if let Some(timer) = &mut res.timer {
        timer.tick(time.delta());
        if timer.finished() {
            res.timer = None;
        }
    }
}

fn handle_kill_message(
    time: Res<Time>,
    players: Query<&BikeMarker, With<Confirmed>>,
    mut kills: ResMut<KillMessages>,
    mut kill_messages: ResMut<Events<MessageEvent<KillMessage>>>,
) {
    for message in kill_messages.drain() {
        if let Ok(bike) = players.get(message.message.killed) {
            let name = bike.name.clone();

            kills.messages.push((
                generate_kill_message(&name),
                Timer::new(KILL_MESSAGE_DURATION, TimerMode::Once),
            ));
        }
    }
    for (_, timer) in &mut kills.messages {
        timer.tick(time.delta());
    }
    kills.messages.retain(|(_, timer)| !timer.finished());
}

fn generate_kill_message(player_name: &str) -> String {
    let verbs = ["flattened", "killed", "trampled", "stomped", "crushed"];
    let verb = verbs.choose(&mut rand::thread_rng()).unwrap();
    format!("You {} {}!", verb, player_name)
}
