use crate::screen::Screen;
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::client::events::MessageEvent;
use lightyear::prelude::client::Predicted;
use lightyear::prelude::{ClientConnectionManager, NetworkTarget};
use shared::network::message::ChatMessage;
use shared::network::protocol::Channel1;
use shared::player::bike::{BikeMarker, ColorComponent};

pub struct ChatPlugin;

pub const CHAT_MESSAGE_DURATION: Duration = Duration::from_secs(5);

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChatMessages {
            open: false,
            current_message: "".to_string(),
            messages: Vec::new(),
        });
        app.add_systems(
            Update,
            (send_chat_system, handle_chat_message).run_if(in_state(Screen::Playing)),
        );
    }
}

#[derive(Resource)]
pub struct ChatMessages {
    /// is chat box open?
    pub(crate) open: bool,
    pub(crate) current_message: String,
    pub(crate) messages: Vec<(ChatMessage, Timer)>,
}

/// Handles sending chat messages
fn send_chat_system(
    keys: Res<ButtonInput<KeyCode>>,
    player: Query<(&ColorComponent, &BikeMarker), With<Predicted>>,
    mut manager: ResMut<ClientConnectionManager>,
    mut chat: ResMut<ChatMessages>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        if chat.open && !chat.current_message.is_empty() {
            if let Ok((color, bike)) = player.get_single() {
                let message = ChatMessage {
                    color: color.0,
                    sender: bike.name.clone(),
                    message: chat.current_message.clone(),
                };
                manager.send_message_to_target::<Channel1, _>(&message, NetworkTarget::All);
                chat.current_message = "".to_string();
            }
        }
        chat.open = !chat.open;
    }
    if keys.just_pressed(KeyCode::Escape) {
        chat.open = false;
        chat.current_message = "".to_string();
    }
}

/// Tick timer for chat messages
fn handle_chat_message(
    time: Res<Time>,
    mut messages: ResMut<ChatMessages>,
    mut events: ResMut<Events<MessageEvent<ChatMessage>>>,
) {
    for message in events.drain() {
        messages.messages.push((
            message.message.clone(),
            Timer::new(CHAT_MESSAGE_DURATION, TimerMode::Once),
        ));
    }
    for (_, timer) in &mut messages.messages {
        timer.tick(time.delta());
    }
    messages.messages.retain(|(_, timer)| !timer.finished());
}
