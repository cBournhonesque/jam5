//! The title screen that appears when the game starts.

use super::Screen;
use crate::audio::sfx::{PlaySfx, SfxKey};
use crate::ui::prelude::*;
use bevy::prelude::*;
use bevy_egui::egui::Margin;
use bevy_egui::{egui, EguiContexts};
use clap::Command;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TitleScreenData {
        name: "".to_string(),
        hovered: false,
    });
    app.add_systems(Update, title.run_if(in_state(Screen::Title)));
}

#[derive(Resource, Default)]
pub struct TitleScreenData {
    pub name: String,
    hovered: bool,
}

fn title(
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    mut title_data: ResMut<TitleScreenData>,
    mut next_screen: ResMut<NextState<Screen>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    let handle_button =
        |button: &egui::Response, title_data: &mut TitleScreenData, commands: &mut Commands| {
            if button.hovered() && !title_data.hovered {
                commands.trigger(PlaySfx::Key(SfxKey::ButtonHover));
            }
            if button.clicked() {
                commands.trigger(PlaySfx::Key(SfxKey::ButtonPress));
            }
        };

    egui::Window::new("Title")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .max_height(500.0)
        .default_height(500.0)
        .min_height(500.0)
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 30.0);
                ui.add_sized(
                    [400.0, 40.0],
                    egui::TextEdit::singleline(&mut title_data.name)
                        .desired_rows(1)
                        .margin(Margin {
                            left: 5.0,
                            top: 10.0,
                            ..default()
                        })
                        .char_limit(20)
                        .desired_width(200.0)
                        .font(egui::FontSelection::FontId(egui::FontId::proportional(
                            24.0,
                        )))
                        .hint_text("Enter your name"),
                );

                let play = ui.button("Play");
                handle_button(&play, title_data.as_mut(), &mut commands);
                if play.clicked() {
                    next_screen.set(Screen::Playing);
                }

                // ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 30.0);
                // ui.add_space(30.0);
                ui.separator();
                let credits = ui.button("Credits");
                handle_button(&credits, title_data.as_mut(), &mut commands);
                if credits.clicked() {
                    next_screen.set(Screen::Credits);
                }

                // exit doesn't work well in embedded applications
                if cfg!(not(target_family = "wasm")) {
                    let exit = ui.button("Exit");
                    handle_button(&exit, title_data.as_mut(), &mut commands);
                    if exit.clicked() {
                        app_exit.send(AppExit::Success);
                    }
                    title_data.hovered = exit.hovered() || play.hovered() || credits.hovered();
                } else {
                    title_data.hovered = play.hovered() || credits.hovered();
                }
            });
        });
}
