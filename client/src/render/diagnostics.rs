use bevy::prelude::*;
use bevy_screen_diagnostics::{Aggregate, ScreenDiagnostics, ScreenDiagnosticsPlugin};
use lightyear::client::prediction::diagnostics::PredictionDiagnosticsPlugin;
use lightyear::shared::ping::diagnostics::PingDiagnosticsPlugin;
use lightyear::transport::io::IoDiagnosticsPlugin;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_diagnostic);
        app.add_plugins(ScreenDiagnosticsPlugin::default());
    }
}

fn setup_diagnostic(mut onscreen: ResMut<ScreenDiagnostics>) {
    onscreen
        .add("Ping".to_string(), PingDiagnosticsPlugin::RTT)
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}ms"));
    onscreen
        .add("Jitter".to_string(), PingDiagnosticsPlugin::JITTER)
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}ms"));
    onscreen
        .add("RB".to_string(), PredictionDiagnosticsPlugin::ROLLBACKS)
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
    onscreen
        .add(
            "RBt".to_string(),
            PredictionDiagnosticsPlugin::ROLLBACK_TICKS,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
    onscreen
        .add(
            "RBd".to_string(),
            PredictionDiagnosticsPlugin::ROLLBACK_DEPTH,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.1}"));
    // screen diagnostics twitches due to layout change when a metric adds or removes
    // a digit, so pad these metrics to 3 digits.
    onscreen
        .add("KB_in".to_string(), IoDiagnosticsPlugin::BYTES_IN)
        .aggregate(Aggregate::Average)
        .format(|v| format!("{v:0>3.0}"));
    onscreen
        .add("KB_out".to_string(), IoDiagnosticsPlugin::BYTES_OUT)
        .aggregate(Aggregate::Average)
        .format(|v| format!("{v:0>3.0}"));
}
