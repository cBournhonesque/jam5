use bevy::app::{App, PluginGroup};
use clap::Parser;
use rand::prelude::IteratorRandom;
use rand::Rng;
use shared::network::config::Transports;
use shared::SharedPlugin;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

pub mod audio;

pub mod camera;
pub mod render;

mod ui;

pub mod assets;
mod inputs;
mod network;
pub mod screen;

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
    app.add_plugins(SharedPlugin { headless: false });
    let client_id = if cli.client_id == 0 {
        rand::thread_rng().gen::<u64>()
    } else {
        cli.client_id
    };
    app.add_plugins(network::NetworkPlugin {
        client_id,
        client_port: cli.client_port,
        server_addr: SocketAddr::new(cli.server_addr.into(), cli.server_port),
        transport: cli.transport,
    });
    app.add_plugins(audio::plugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(inputs::InputPlugin);
    app.add_plugins(screen::plugin);
    app.add_plugins(render::RenderPlugin);
    app
}
