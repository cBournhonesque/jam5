use bevy::prelude::*;
use clap::Parser;

use shared::network::config::Transports;
use shared::SharedPlugin;

mod game;
mod network;
mod player;

pub const SERVER_PORT: u16 = 5000;

#[derive(Parser, PartialEq, Debug)]
pub struct Cli {
    #[arg(short, long, default_value = "false")]
    render: bool,

    #[arg(short, long, default_value = "false")]
    inspector: bool,

    #[arg(short, long, default_value_t = SERVER_PORT)]
    port: u16,

    #[arg(short, long, value_enum, default_value_t = Transports::WebTransport)]
    transport: Transports,
}

pub fn app(cli: Cli) -> App {
    let mut app = App::new();
    app.add_plugins(SharedPlugin {
        headless: !cli.render,
    });

    // game
    app.add_plugins(game::start::GamePlugin);

    // networking
    app.add_plugins(network::NetworkPlugin {
        server_port: cli.port,
        transport: cli.transport,
    });

    // player
    app.add_plugins(player::PlayerPlugin);
    app
}
