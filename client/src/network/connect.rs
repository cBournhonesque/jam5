use bevy::prelude::*;
use lightyear::prelude::client::*;
use shared::network::message::SpawnPlayerMessage;
use shared::network::protocol::Channel1;

/// Send message to server on connect with the player name
pub fn on_connect(mut manager: ResMut<ConnectionManager>) {
    let _ = manager.send_message::<Channel1, _>(&SpawnPlayerMessage {
        name: "Player".to_string(),
    });
}
