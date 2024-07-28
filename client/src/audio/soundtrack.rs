use crate::assets::{AssetKey, HandleMap};
use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => *key,
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Key(SoundtrackKey),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            // (
            //     SoundtrackKey::Credits,
            //     asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            // ),
            // (
            //     SoundtrackKey::Gameplay,
            //     asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            // ),
        ]
        .into()
    }
}
