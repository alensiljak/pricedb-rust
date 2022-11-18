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
#[tokio::main]
async fn main() {
    // initialize logging
    env_logger::init();
    log::trace!("starting");

    let cli = Cli::parse();

    log::debug!("Command: {:?}", cli.command);

    let app = App::new();

    match &cli.command {
        Some(Commands::Dl {
            currency,
            agent,
            exchange,
            symbol,
        }) => app.download_prices(exchange, symbol, currency, agent),
        
        Some(Commands::Prune { symbol }) => app.prune(symbol),
        
        _ => println!("No command issued"),
    }
}
