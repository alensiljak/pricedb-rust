mod interface;

use clap::Parser;
use interface::{Cli, Commands};
use tracing::{debug, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    trace!("In download...");

    match currency_option {
        Some(currency) => debug!("currency option: {}", currency),
        None => println!("no currency")
    }

    // todo: download prices
}

fn prune(symbol: &Option<String>) {
    trace!("In prune. Incomplete. symbol: {:?}", symbol);
}

fn initialize_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
