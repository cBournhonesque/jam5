use crate::assets::{AssetKey, HandleMap};
use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    BikeDeath,
    BikeSound,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/Menu2.mp3"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/Menu1.mp3"),
            ),
            (SfxKey::BikeDeath, asset_server.load("audio/sfx/Die1.mp3")),
            // (
            //     SfxKey::BikeSound,
            //     asset_server.load("audio/sfx/Engine4Bass.mp3"),
            // ),
        ]
        .into()
    }
}
