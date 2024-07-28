use crate::assets::HandleMap;
use crate::audio::sfx::{PlaySfx, SfxKey};
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleBurst,
    ParticleSystem, ParticleSystemBundle, Playing, VelocityModifier,
};
use lightyear::prelude::client::*;
use rand::prelude::SliceRandom;
use shared::network::message::{BikeDeathMessage, KillMessage, KilledByMessage};
use shared::player::bike::{BikeMarker, ColorComponent};
use shared::player::death::DEATH_TIMER;
use shared::player::scores::Stats;
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
        app.add_systems(
            Update,
            (
                handle_kill_message,
                handle_killed_by_message,
                handle_death_message,
            ),
        );
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

fn handle_death_message(
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
    mut messages: ResMut<Events<MessageEvent<BikeDeathMessage>>>,
) {
    for message in messages.drain() {
        let color = message.message.color;
        let position = message.message.position;
        commands.spawn((
            GlobalTransform::from_translation(position.extend(0.).into()),
            AudioBundle {
                source: sfx_handles[&SfxKey::BikeDeath].clone_weak(),
                settings: PlaybackSettings::DESPAWN.with_spatial(true),
            },
        ));
        commands.spawn((
            ParticleSystemBundle {
                transform: Transform::from_translation(position.extend(100.)),
                particle_system: ParticleSystem {
                    lifetime: JitteredValue::jittered(0.85, -0.50..0.05),
                    spawn_rate_per_second: 0.0.into(),
                    max_particles: 3_00,
                    initial_speed: JitteredValue::jittered(500.0, -400.0..400.0),
                    initial_scale: JitteredValue::jittered(5.0, -4.0..4.0),
                    scale: (1.0..0.0).into(),
                    emitter_shape: EmitterShape::CircleSegment(CircleSegment {
                        opening_angle: std::f32::consts::PI * 0.6,
                        direction_angle: std::f32::consts::PI * 0.5,
                        ..default()
                    }),
                    velocity_modifiers: vec![
                        VelocityModifier::Drag(0.001.into()),
                        VelocityModifier::Vector(Vec3::new(0.0, -400.0, 0.0).into()),
                    ],
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new((color.to_linear() * 5.0).into(), 0.0),
                        CurvePoint::new((color.to_linear() * 1.0).into(), 1.0),
                    ])),
                    system_duration_seconds: 0.2,
                    bursts: vec![
                        ParticleBurst {
                            time: 0.0,
                            count: 100,
                        },
                        ParticleBurst {
                            time: 0.1,
                            count: 100,
                        },
                        ParticleBurst {
                            time: 0.2,
                            count: 100,
                        },
                    ],
                    looping: false,
                    despawn_on_finish: true,
                    ..ParticleSystem::oneshot()
                },
                ..default()
            },
            Playing,
            Name::from("KillParticles"),
        ));
    }
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
