use bevy::prelude::{Component, Reflect};
use lightyear::prelude::{ClientId, Deserialize, Serialize};

pub mod bike;

pub mod trail;
pub mod zone;
pub mod scores;
pub mod death;

/// Indicates a player entity
/// TODO: should we separate the player entity from the actual visual character?
///  the character could be dead while the player is still connected
///  or we could just an Alive/Dead component
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq, Reflect)]
pub struct Player {
    pub id: ClientId,
    pub name: String,
}