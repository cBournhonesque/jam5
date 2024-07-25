//! The title screen that appears when the game starts.

use super::Screen;
use crate::ui::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerNamePrompt {
        name: "".to_string(),
    });
    app.add_systems(PostUpdate, title.run_if(in_state(Screen::Title)));
}

#[derive(Resource, Default)]
pub struct PlayerNamePrompt {
    pub name: String,
}

fn title(
    mut egui_contexts: EguiContexts,
    mut player_name_prompt: ResMut<PlayerNamePrompt>,
    mut next_screen: ResMut<NextState<Screen>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    egui::Window::new("Title")
        .title_bar(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .max_height(200.0)
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 10.0);
                ui.add(
                    egui::TextEdit::singleline(&mut player_name_prompt.name)
                        .char_limit(20)
                        .desired_width(200.0)
                        .hint_text("Enter your name"),
                );
                if ui.button("Play").clicked() {
                    next_screen.set(Screen::Playing);
                }

                ui.separator();
                if ui.button("Credits").clicked() {
                    next_screen.set(Screen::Credits);
                }
                // exit doesn't work well in embedded applications
                #[cfg(not(target_family = "wasm"))]
                if ui.button("Exit").clicked() {
                    app_exit.send(AppExit::Success);
                }
            });
        });
}
