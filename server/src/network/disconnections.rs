use crate::network::connections::AvailableColors;
use bevy::prelude::*;
use lightyear::prelude::ServerDisconnectEvent;
use shared::player::bike::{BikeMarker, ColorComponent};

// NOTE: we cannot use Trigger<DisconnectEvent> because we have an observer
pub(crate) fn observe_disconnect(
    trigger: Trigger<OnRemove, ColorComponent>,
    bikes: Query<&ColorComponent, With<BikeMarker>>,
    mut colors: ResMut<AvailableColors>,
) {
    if let Ok(color) = bikes.get(trigger.entity()) {
        info!("Player disconnected: {:?}", color.0);
        colors.add_color(color.0);
    }
}
