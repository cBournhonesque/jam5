use clap::Parser;

use client::{app, Cli};

fn main() {
    let cli = Cli::parse();
    let mut app = app(cli);
    app.run();
}