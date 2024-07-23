use bevy::prelude::*;
use lightyear::prelude::*;

/// Current score of a player
#[derive(Reflect, Component, Serialize, Deserialize, PartialEq, Eq, Default, Debug, Clone, PartialOrd, Ord)]
pub struct Score {
    pub score: u32,
}

/// Stats of a player
#[derive(Reflect, Component, Serialize, Deserialize, Default, PartialEq, Debug, Clone)]
pub struct Stats {
    pub kills: u32,
    pub time_lived_secs: u32,
    pub max_area: u32,
    pub max_zones: u32,
    pub max_trail_length: u32,
    pub max_score: u32,
}