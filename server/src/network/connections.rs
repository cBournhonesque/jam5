//! Handle client connections

use avian2d::prelude::RigidBody;
use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use shared::player::bike::BikeBundle;

/// Spawn a new bike when a player connects
pub(crate) fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    info!("Spawning bike for client {}", trigger.event().client_id);
    let client_id = trigger.event().client_id;
    let color = color_from_client_id(client_id.to_bits());
    commands.spawn((
        BikeBundle::new_at(client_id.to_bits(), Vec2::new(0.0, 0.0), color),
        RigidBody::Kinematic,
        Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            // TODO: add network relevance
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            ..default()
        },
    ));
}

fn color_from_client_id(client_id: u64) -> Color {
    let available_colors = [
        css::LIMEGREEN,
        css::PINK,
        css::YELLOW,
        css::AQUA,
        css::CRIMSON,
        css::GOLD,
        css::ORANGE_RED,
        css::SILVER,
        css::SALMON,
        css::YELLOW_GREEN,
        css::WHITE,
        css::RED,
    ];
    let color = available_colors
        .get(client_id as usize % available_colors.len())
        .unwrap();
    return Color::Srgba(*color);
}
