//! Display a leaderboard with player scores
use crate::screen::Screen::Playing;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_extras::{Column, TableBuilder};
use shared::player::bike::BikeMarker;
use shared::player::scores::Score;

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(PostUpdate, leaderboard_ui.run_if(in_state(Playing)));
    }
}

fn leaderboard_ui(mut contexts: EguiContexts, scores: Query<(&Score, &BikeMarker)>) {
    let scores = scores
        .iter()
        .sort_by::<(&Score, &BikeMarker)>(|(a, _), (b, _)| b.cmp(a))
        .map(|(score, bike)| (bike.name.clone(), score.total()))
        .take(6)
        .collect::<Vec<_>>();
    egui::Window::new("Leaderboard")
        .anchor(egui::Align2::RIGHT_TOP, [30.0, 30.0])
        .show(contexts.ctx_mut(), |ui| {
            let table = TableBuilder::new(ui)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto());
            table
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Score");
                    });
                })
                .body(|mut body| {
                    for (name, score) in scores.iter() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                ui.label(score.to_string());
                            });
                        });
                    }
                })
        });
}
