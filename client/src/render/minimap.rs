use crate::render::egui::BG_COLOR;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct MinimapPlugin;

// const MINIMAP_COLOR: Color = Color::Srgba(Srgba::rgba_u8(0, 36, 42, 50));

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, draw_map_egui);
    }
}

fn draw_map_egui(mut egui_ctx: EguiContexts) {
    egui::Window::new("Minimap")
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-100.0, -100.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Minimap");
            ui.painter()
                .circle_filled(egui::Pos2::new(100.0, 100.0), 100.0, BG_COLOR);
        });
}
