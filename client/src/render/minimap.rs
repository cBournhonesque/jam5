use crate::render::egui::BG_COLOR;
use crate::screen::Screen::Playing;
use avian2d::prelude::Rotation;
use bevy::prelude::*;
use bevy_egui::egui::emath::RectTransform;
use bevy_egui::egui::{Pos2, Sense};
use bevy_egui::{egui, EguiContexts};
use lightyear::prelude::client::{Confirmed, Interpolated, Predicted};
use shared::map::MAP_SIZE;
use shared::player::bike::{BikeMarker, ColorComponent};

pub struct MinimapPlugin;

const MINIMAP_SIZE: f32 = 100.0;

// const MINIMAP_COLOR: Color = Color::Srgba(Srgba::rgba_u8(0, 36, 42, 50));

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, draw_map_egui.run_if(in_state(Playing)));
    }
}

fn draw_map_egui(
    mut egui_ctx: EguiContexts,
    players: Query<
        (&ColorComponent, &Transform),
        (Or<(With<Interpolated>, With<Predicted>)>, With<BikeMarker>),
    >,
) {
    egui::Window::new("Minimap")
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .interactable(false)
        .frame(egui::Frame::none())
        .show(egui_ctx.ctx_mut(), |ui| {
            let (response, painter) = ui.allocate_painter(
                bevy_egui::egui::Vec2::new(2.0 * MINIMAP_SIZE, 2.0 * MINIMAP_SIZE),
                Sense::hover(),
            );

            // Get the relative position of our "canvas"
            let to_screen = RectTransform::from_to(
                egui::Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );
            painter.circle_filled(
                to_screen.transform_pos(Pos2::new(MINIMAP_SIZE, MINIMAP_SIZE)),
                MINIMAP_SIZE,
                BG_COLOR,
            );

            for (color, transform) in players.iter() {
                // transform the position from world coordinates to the minimap size
                let mut vec = transform.translation.truncate();
                // The ui is flipped on the y axis
                vec.y = -vec.y;
                let vec_mapped = vec / MAP_SIZE * MINIMAP_SIZE + Vec2::new(100.0, 100.0);
                error!("vec_mapped: {:?}", vec_mapped);
                let rad = transform.rotation.to_euler(EulerRot::ZXY).0;
                let rot = Rotation::radians(rad);
                painter.arrow(
                    to_screen.transform_pos(Pos2::new(vec_mapped.x, vec_mapped.y)),
                    // the ui is flipped on the y axis
                    egui::Vec2::new(rot.cos, -rot.sin) * 5.0,
                    egui::Stroke::new(1.0, egui::Color32::from(color)),
                );
            }
        });
}
