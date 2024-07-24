use bevy::prelude::*;
use lightyear::prelude::client::*;
use shared::network::message::KillMessage;

pub struct KillPlugin;

impl Plugin for KillPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

fn handle_kill_message(kills: EventReader<MessageEvent<KillMessage>>) {
    // for (killed, kill_message) in kills.iter_mut() {
    // TODO: spawn a message box
    // }
}
