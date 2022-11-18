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

//#[async_std::main]
fn main() {
    initialize_logging();

    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);

    match &cli.command {
        Some(Commands::Dl { currency }) => download(currency),
        Some(Commands::Prune { symbol }) => prune(symbol),
        _ => println!("No command issued"),
    }
}

fn download(currency_option: &Option<String>) {
    //trace!("In download...");

    let mut options = app::DownloadOptions {
        exchange: None,
        mnemonic: None,
        currency: None,
        agent: None,
    };

    match currency_option {
        Some(currency) => options.currency = Some(currency.to_string()),
        None => println!("no currency"),
    }

    // download prices
    app::download_prices(options);
}

fn prune(symbol: &Option<String>) {
    trace!("In prune. Incomplete. symbol: {:?}", symbol);
}

fn initialize_logging() {
    fast_log::init(Config::new().console()).unwrap();
}
