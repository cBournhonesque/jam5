//! Display UI via egui. All windows displayed must be in a single system.

use crate::render::chat::ChatMessages;
use crate::render::kills::{KillMessages, KilledByMessageRes};
use crate::screen::Screen::Playing;
use avian2d::prelude::Position;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{FontId, Frame, Margin, RichText};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_extras::{Column, TableBuilder};
use lightyear::client::prediction::Predicted;
use lightyear::shared::replication::components::Controlled;
use shared::map::MAP_SIZE;
use shared::physics::movement::{MAP_EDGE_SLOW_ZONE, TRAIL_SIZE_SLOW_START};
use shared::player::bike::{BikeMarker, ClientIdMarker, ColorComponent};
use shared::player::scores::Score;
use shared::player::trail::Trail;

pub struct MyEguiPlugin;

const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(11, 170, 173, 50);

const TITLE_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(22, 255, 255, 50);

pub const BG_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 36, 42, 50);

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
    // style.visuals.override_text_color = Some(text_color);

    let button_bg_color = egui::Color32::from_rgba_premultiplied(113, 136, 173, 50);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    style.visuals.widgets.inactive.fg_stroke.color = text_color;
    style.visuals.widgets.inactive.bg_fill = button_bg_color;
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;

    style.visuals.window_stroke = egui::Stroke::NONE;
    style.visuals.window_fill = BG_COLOR;
    style.visuals.window_shadow = egui::Shadow::NONE;
    style.visuals.popup_shadow.color = BG_COLOR;
    style.visuals.popup_shadow = egui::Shadow::NONE;
    style.visuals.clip_rect_margin = 0.0;

    style.spacing.window_margin = egui::Margin::same(20.0);

    // widget visuals
    egui_ctx.ctx_mut().set_style(style);
}

fn leaderboard_ui(
    mut egui_contexts: EguiContexts,
    killed_by: Res<KilledByMessageRes>,
    kills: Res<KillMessages>,
    mut chat: ResMut<ChatMessages>,
    scores: Query<(&Score, &BikeMarker, &ColorComponent), With<BikeMarker>>,
    predicted_bike: Query<&Position, (With<Predicted>, With<BikeMarker>)>,
    controlled_trail: Query<&Trail, With<Controlled>>,
) {
    // Chat window
    if !chat.messages.is_empty() {
        egui::Window::new("Chat")
            .title_bar(false)
            .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -200.0])
            .min_width(100.0)
            .max_width(300.0)
            .show(egui_contexts.ctx_mut(), |ui| {
                for (message, _) in &chat.messages {
                    ui.label(
                        RichText::new(format!("[{}]: {}", message.sender, message.message))
                            .color(&ColorComponent(message.color))
                            .font(FontId::proportional(16.0)),
                    );
                    // ui.label(message.message.to_string());
                }
            });
    }
    if chat.open {
        egui::Window::new("ChatInput")
            .title_bar(false)
            .frame(egui::Frame::none())
            .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -150.0])
            .show(egui_contexts.ctx_mut(), |ui| {
                ui.text_edit_singleline(&mut chat.current_message)
                    .request_focus();
            });
    }

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

    // Slow reasons
    if let Ok(pos) = predicted_bike.get_single() {
        if pos.0.length() > MAP_SIZE - MAP_EDGE_SLOW_ZONE {
            egui::Window::new("SlowZone")
                .title_bar(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -20.0])
                .show(egui_contexts.ctx_mut(), |ui| {
                    ui.label("The edge of the map is a slow zone!");
                });
        }
    }
    if let Ok(trail) = controlled_trail.get_single() {
        if trail.len() > TRAIL_SIZE_SLOW_START {
            egui::Window::new("SlowTrail")
                .title_bar(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -80.0])
                .show(egui_contexts.ctx_mut(), |ui| {
                    ui.label("Your trail is too long! It is slowing you down!");
                });
        }
    }

    // leaderboard
    let scores = scores
        .iter()
        .sort_by::<(&Score, &BikeMarker, &ColorComponent)>(|(a, _, _), (b, _, _)| {
            (b.kill_score, b.zone_score).cmp(&(a.kill_score, a.zone_score))
        })
        // .map(|(score, bike, color)| (bike.name.clone(), score.clone()))
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
                        let mut text: RichText = "Kills".into();
                        text = text.color(TITLE_COLOR);
                        ui.strong(text);
                    });
                    header.col(|ui| {
                        let mut text: RichText = "Area".into();
                        text = text.color(TITLE_COLOR);
                        ui.strong(text);
                    });
                })
                .body(|mut body| {
                    for (score, name, color) in scores.iter() {
                        body.row(30.0, |mut row| {
                            let color = ColorComponent(color.0.with_alpha(0.9));
                            row.col(|ui| {
                                ui.label(RichText::new(name.name.to_string()).color(&color));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(score.kill_score.to_string()).color(&color));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(score.zone_score.to_string()).color(&color));
                            });
                        });
                    }
                })
        });
}
