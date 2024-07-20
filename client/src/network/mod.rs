//! Module to handle all networking-related logic
use std::net::SocketAddr;
use bevy::prelude::*;
use lightyear::prelude::client::*;

use shared::network::config::Transports;

pub(crate) mod config;
// mod connect;

/// Plugin that handles networking
pub(crate) struct NetworkPlugin {
    pub(crate) client_id: u64,
    pub(crate) client_port: u16,
    pub(crate) server_addr: SocketAddr,
    pub(crate) transport: Transports,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // the ClientPlugins must be added before the Protocol plugins
        app.add_plugins(config::build_lightyear_plugins(
            self.client_id,
            self.client_port,
            self.server_addr,
            self.transport,
        ));

        app.add_plugins(shared::network::protocol::ProtocolPlugin);
        // app.add_plugins(NetworkInputsPlugin);
        // app.add_plugins(InterpolationPlugin);

        // TODO: make this state-scoped
        app.add_systems(Startup, connect);
    }
}

/// Connect to the server
fn connect(mut commands: Commands) {
    commands.connect_client();
}