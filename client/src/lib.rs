use std::net::Ipv4Addr;

use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::log::{Level, LogPlugin};
use clap::Parser;

use shared::network::config::Transports;
use shared::network::protocol::ProtocolPlugin;
use shared::SharedPlugin;

mod audio;

mod render;
mod debug;
mod collision;
mod camera;
mod inputs;
mod menu;

pub mod screen;
mod network;
mod camera;
mod render;
mod assets;

// Use a port of 0 to automatically select a port
pub const CLIENT_PORT: u16 = 0;

pub const SERVER_PORT: u16 = 5000;


#[derive(Parser, PartialEq, Debug)]
pub struct Cli {
    #[arg(short, long, default_value = "false")]
    inspector: bool,

    #[arg(short, long, default_value_t = 0)]
    client_id: u64,

    #[arg(long, default_value_t = CLIENT_PORT)]
    client_port: u16,

    #[arg(long, default_value_t = Ipv4Addr::LOCALHOST)]
    server_addr: Ipv4Addr,

    #[arg(short, long, default_value_t = SERVER_PORT)]
    server_port: u16,

    #[arg(short, long, value_enum, default_value_t = Transports::WebTransport)]
    transport: Transports,
}

pub fn app(cli: Cli) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
        custom_layer: |_| None,
    }));

    app.add_plugins(network::NetworkPlugin {
        client_id: cli.client_id,
        client_port: cli.client_port,
        server_addr: (cli.server_addr, cli.server_port).into(),
        transport: cli.transport,
    });
    app.add_plugins(audio::plugin);
    app.add_plugins(screen::plugin);
    app.add_plugins(inputs::LocalInputsPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(collision::CollisionPlugin);
    app.add_plugins(debug::DebugPlugin);
    app.add_plugins(render::RenderPlugin);
    app.add_plugins(SharedPlugin);
    app
}