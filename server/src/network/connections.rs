//! Handle client connections

use avian2d::math::Vector;
use avian2d::prelude::RigidBody;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::reflect::ApplyError;
use lightyear::prelude::*;
use lightyear::prelude::server::*;
use shared::player::bike::BikeBundle;
use shared::player::trail::TrailBundle;



/// Spawn a new bike when a player connects
pub(crate) fn spawn_bike(trigger: Trigger<ConnectEvent>, mut commands: Commands, ) {
    info!("Spawning bike for client {}", trigger.event().client_id);
    let client_id = trigger.event().client_id;
    let color = color_from_client_id(client_id.to_bits());
    let pos = Vector::default();
    commands.spawn((
        BikeBundle::new_at(pos, color),
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
    )).with_children(|parent| {
        // this might not be needed since I think lightyear hierarchy does it already,
        // but make sure that the whole hierarchy is replicated in the same group
        let group = ReplicationGroup::new_id(parent.parent_entity().to_bits());
        parent.spawn((
            TrailBundle::new_at(pos),

            // TODO: do we need to add Replicate here? Also the children have the same SyncTarget as the parent, which might not
            //  be what we want here..
            // Replicate {
            //     // TODO: currently no prediction or interpolation for the trail. But maybe we would like to predict it on the client?
            //     //  just so that it matches visually?
            //     // TODO: add network relevance
            //     controlled_by: ControlledBy {
            //         target: NetworkTarget::Single(client_id),
            //         ..default()
            //     },
            //     group,
            //     ..default()
            // },
        ));
    });
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
    let color = available_colors.get(client_id as usize % available_colors.len()).unwrap();
    return Color::Srgba(*color);
}