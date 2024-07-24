use bevy::prelude::*;
use lightyear::prelude::*;

/// Current score of a player
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
pub struct Score {
    pub zone_score: u32,
    pub kill_score: u32,
}

impl Score {
    pub fn total(&self) -> u32 {
        self.zone_score + self.kill_score
    }
}

/// Stats of a player (shown when they die)
#[derive(Reflect, Component, Serialize, Deserialize, Default, PartialEq, Debug, Clone)]
pub struct Stats {
    pub kills: u32,
    pub time_lived_secs: u32,
    pub max_area: u32,
    pub max_zones: u32,
    pub max_trail_length: u32,
    pub max_score: u32,
}
