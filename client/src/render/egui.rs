//! Display UI via egui. All windows displayed must be in a single system.
use crate::render::kills::{KillMessages, KilledByMessageRes};
use crate::screen::Screen::Playing;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{FontId, RichText};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use egui_extras::{Column, TableBuilder};
use shared::player::bike::{BikeMarker, ClientIdMarker};
use shared::player::scores::Score;

pub struct MyEguiPlugin;

const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(11, 170, 173, 50);

const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(22, 255, 255, 50);

const BG_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 36, 42, 50);

impl Plugin for MyEguiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Startup, global_egui_visuals);
        app.add_systems(Update, leaderboard_ui.run_if(in_state(Playing)));
    }
}

fn global_egui_visuals(mut egui_ctx: EguiContexts) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Autobus".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Autobusbold-1ynL.ttf")),
    );
    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "Autobus".to_owned());
    egui_ctx.ctx_mut().set_fonts(fonts);

    let mut style = egui::Style::default();

    // container visuals
    *style.text_styles.get_mut(&egui::TextStyle::Button).unwrap() =
        egui::FontId::new(24.0, Proportional);
    *style.text_styles.get_mut(&egui::TextStyle::Body).unwrap() =
        egui::FontId::new(16.0, Proportional);

    let text_color = egui::Color32::from_rgba_premultiplied(11, 170, 173, 50);
    style.visuals.override_text_color = Some(text_color);

    let bg_color = egui::Color32::from_rgba_premultiplied(0, 36, 42, 50);
    let button_bg_color = egui::Color32::from_rgba_premultiplied(113, 136, 173, 50);
    style.visuals.widgets.inactive.bg_fill = button_bg_color;
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

    style.visuals.window_stroke = egui::Stroke::NONE;
    style.visuals.window_fill = bg_color;
    style.visuals.window_shadow = egui::Shadow::NONE;
    style.visuals.popup_shadow.color = bg_color;
    style.visuals.popup_shadow = egui::Shadow::NONE;
    style.visuals.clip_rect_margin = 0.0;

    style.spacing.window_margin = egui::Margin::symmetric(20.0, 20.0);

    // widget visuals
    egui_ctx.ctx_mut().set_style(style);
}

fn leaderboard_ui(
    mut egui_contexts: EguiContexts,
    killed_by: Res<KilledByMessageRes>,
    kills: Res<KillMessages>,
    scores: Query<(&Score, &BikeMarker), With<BikeMarker>>,
) {
    // Killed by window
    if let Some(timer) = &killed_by.timer {
        egui::Window::new("KilledBy")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui_contexts.ctx_mut(), |ui| {
                ui.label(&killed_by.message);
                ui.add_space(30.0);
                ui.separator();
                ui.add_space(30.0);
                ui.label("Stats:\n");
                let stats = &killed_by.stats;
                ui.label(format!("Kills: {}", stats.kills));
                ui.label(format!("Time lived: {}s", stats.time_lived_secs));
                ui.label(format!("Max area: {}", stats.max_area));
                ui.label(format!("Max zones: {}", stats.max_zones));
                ui.label(format!("Max trail length: {}", stats.max_trail_length));
                ui.label(format!("Max score: {}", stats.max_score));

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(30.0);

                ui.label(format!(
                    "Respawn in {:.0}...",
                    timer.duration().as_secs_f32() - timer.elapsed().as_secs_f32()
                ));
            });
    }

    // kill messages
    if !kills.messages.is_empty() {
        egui::Window::new("Killed")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 200.0])
            .show(egui_contexts.ctx_mut(), |ui| {
                for (message, _) in &kills.messages {
                    ui.label(RichText::new(message).font(FontId::proportional(16.0)));
                }
            });
    }

    // leaderboard
    let scores = scores
        .iter()
        .sort_by::<(&Score, &BikeMarker)>(|(a, _), (b, _)| b.cmp(a))
        .map(|(score, bike)| (bike.name.clone(), score.total()))
        .take(6)
        .collect::<Vec<_>>();
    egui::Window::new("Leaderboard")
        .anchor(egui::Align2::RIGHT_TOP, [30.0, 30.0])
        .title_bar(false)
        .show(egui_contexts.ctx_mut(), |ui| {
            let table = TableBuilder::new(ui)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto());
            table
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        let mut text: RichText = "Name".into();
                        text = text.color(TITLE_COLOR);
                        ui.strong(text);
                    });
                    header.col(|ui| {
                        let mut text: RichText = "Score".into();
                        text = text.color(TITLE_COLOR);
                        ui.strong(text);
                    });
                })
                .body(|mut body| {
                    for (name, score) in scores.iter() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(name.to_string());
                            });
                            row.col(|ui| {
                                ui.label(score.to_string());
                            });
                        });
                    }
                })
        });
}
