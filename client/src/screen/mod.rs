//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod playing;
mod splash;
pub mod title;

#[cfg(feature = "dev")]
use bevy::dev_tools::states::log_transitions;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_systems(Update, log_transitions::<Screen>);
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    Splash,
    Loading,
    #[default]
    Title,
    Credits,
    Playing,
}
