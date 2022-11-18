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
use fast_log::Config;
use interface::{Cli, Commands};
use log::{debug, trace};

use crate::app::App;

//#[async_std::main]
fn main() {
    initialize_logging();

    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);

    let app = App {};

    match &cli.command {
        Some(Commands::Dl { currency,
        agent, exchange, 
        symbol }) => app.download_prices(exchange, symbol, currency, agent),
        Some(Commands::Prune { symbol }) => prune(symbol),
        _ => println!("No command issued"),
    }
}

fn prune(symbol: &Option<String>) {
    trace!("In prune. Incomplete. symbol: {:?}", symbol);
}

fn initialize_logging() {
    fast_log::init(Config::new().console()).unwrap();
}
