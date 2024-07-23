/// Server networking related plugins
mod config;
pub mod connections;

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

        // systems
        app.add_systems(Startup, start_server);
        app.observe(connections::spawn_bike);
    }
}

fn start_server(mut commands: Commands) {
    commands.start_server();
}
