use clap::Parser;
use std::env;

use client::{app, Cli};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let cli = Cli::parse();
    let mut app = app(cli);
    app.run();
}
