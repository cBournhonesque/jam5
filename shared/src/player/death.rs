//! Components related to the death of a player

use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::*;

pub const DEATH_TIMER: Duration = Duration::from_secs(5);

#[derive(
    Reflect,
    Component,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    Debug,
    Clone,
    PartialOrd,
    Ord,
)]
pub struct Dead;

#[derive(Reflect, Component, Default, Debug, Clone)]
pub struct DeathTimer {
    pub respawn_timer: Timer,
}

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DeathTimer>().register_type::<Dead>();
    }
}
