use clap::Parser;
use server::app;
use server::Cli;

fn main() {
    let cli = Cli::parse();
    let mut app = app(cli);
    app.run();
}