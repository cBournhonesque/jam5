//! Module to handle all networking-related logic
use bevy::prelude::*;
use lightyear::prelude::client::*;
use std::net::SocketAddr;

use crate::network::connect::on_connect;
use crate::screen::Screen::Playing;
use shared::network::config::Transports;

mod bike;
pub(crate) mod config;
mod connect;

/// Plugin that handles networking
pub(crate) struct NetworkPlugin {
    pub(crate) client_id: u64,
    pub(crate) client_port: u16,
    pub(crate) server_addr: SocketAddr,
    pub(crate) transport: Transports,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<BikeSpawned>();
        // the ClientPlugins must be added before the Protocol plugins
        app.add_plugins(config::build_lightyear_client(
            self.client_id,
            self.client_port,
            self.server_addr,
            self.transport,
        ));

        app.add_plugins(shared::network::protocol::ProtocolPlugin);

        app.add_plugins(bike::BikeNetworkPlugin);

        app.add_systems(OnEnter(Playing), connect.run_if(not(is_connected)));
        app.add_systems(OnEnter(NetworkingState::Connected), on_connect);

        #[cfg(feature = "dev")]
        app.observe(debug_connect);
    }
}

/// Connect to the server
fn connect(mut commands: Commands) {
    commands.connect_client();
}

/// Show the client id when connected
#[cfg(feature = "dev")]
fn debug_connect(trigger: Trigger<ConnectEvent>, mut commands: Commands) {
    info!("Client connected: {}", trigger.event().client_id());
    let client_id = trigger.event().client_id();
    commands.spawn((
        TextBundle::from_section(
            format!("Client {}", client_id),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        Name::new("ClientIdText"),
    ));
}

#[derive(Event, Debug)]
pub struct BikeSpawned {
    pub(crate) color: Color,
    pub(crate) entity: Entity,
}
