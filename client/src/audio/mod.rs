pub mod sfx;
pub mod soundtrack;

use bevy::audio::{DefaultSpatialScale, SpatialScale};
use bevy::prelude::*;

// 100 pixel = 1 unit of audio
pub const SPATIAL_SCALE: f32 = 1.0 / 150.0;

pub fn plugin(app: &mut App) {
    app.insert_resource(DefaultSpatialScale(SpatialScale::new(0.01)));
    app.add_plugins(sfx::plugin);
    // app.add_plugins((sfx::plugin, soundtrack::plugin));
}
