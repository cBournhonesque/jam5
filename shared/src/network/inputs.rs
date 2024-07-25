use bevy::prelude::Reflect;
use leafwing_input_manager::{
    axislike::{DualAxisData, SingleAxis},
    input_map::InputMap,
    Actionlike,
};
use lightyear::prelude::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerMovement {
    MousePositionRelative,
    Pause,
}
