/// Server networking related plugins
mod config;
pub mod connections;
pub mod disconnections;

use bevy::prelude::*;
use lightyear::prelude::server::*;

use shared::network::config::Transports;
use shared::network::protocol::ProtocolPlugin;

pub struct NetworkPlugin {
    pub server_port: u16,
    pub transport: Transports,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // plugins
        app.add_plugins(config::build_lightyear_server(
            self.server_port,
            self.transport,
        ));
        app.add_plugins(ProtocolPlugin);

        // resources
        app.init_resource::<connections::AvailableColors>();

        // systems
        app.add_systems(Startup, start_server);
        app.add_systems(Update, connections::spawn_bike);
        app.observe(disconnections::observe_disconnect);
    }
}

fn start_server(mut commands: Commands) {
    commands.start_server();
}
