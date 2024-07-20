use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;
use lightyear::prelude::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerMovement {
    MoveCursor,
}