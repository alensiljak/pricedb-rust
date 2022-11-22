/*
 * Price Database
 * The main file contains only the CLI definition.
 */
mod app;
mod config;
mod database;
mod interface;
mod model;

use clap::Parser;
use interface::{Cli, Commands};

use crate::app::App;

//#[async_std::main]
// #[tokio::main]
fn main() {
    // initialize logging
    env_logger::init();
    log::trace!("starting");

    let cli = Cli::parse();

    log::debug!("Command: {:?}", cli.command);

    let app = App::new();

    if cli.command.is_none() {
        println!("No command issued");
        return;
    }

    // download
    if let Some(Commands::Dl {
        currency,
        agent,
        exchange,
        symbol,
    }) = &cli.command {
        app.download_prices(exchange, symbol, currency, agent);
    }

    // prune
    if let Some(Commands::Prune { symbol }) = &cli.command {
        app.prune(symbol);
    }
}
