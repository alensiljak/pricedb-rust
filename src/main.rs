/**
 * Price Database
 * 
 * Retrieving, storing, and exporting commodity prices in Ledger format
 * 
 * See [Documentation](https://github.com/alensiljak/pricedb-rust).
 */

/*
 * The main file contains only the CLI definition.
 */
mod interface;

use clap::Parser;
use interface::{Cli, Commands};
use pricedb::model::SecurityFilter;

//#[async_std::main]
#[tokio::main]
async fn main() {
    // initialize logging
    env_logger::init();
    log::trace!("starting");

    let args = Cli::parse();

    log::debug!("Command: {:?}", args.command);
    let cfg = pricedb::load_config();
    let app = pricedb::App::new(cfg);

    // config

    if let Some(Commands::Config(interface::ConfigCmd::Show)) = &args.command {
        app.config_show();
    }

    // export

    if let Some(Commands::Export {}) = &args.command {
        app.export();
    }

    // download

    if let Some(Commands::Dl {
        currency,
        agent,
        exchange,
        symbol,
    }) = &args.command
    {
        let filter = SecurityFilter {
            currency: currency.clone(),
            agent: agent.clone(),
            exchange: exchange.clone(),
            symbol: symbol.clone(),
        };
        app.download_prices(filter).await;
    }

    // list

    if let Some(Commands::List {
        date,
        currency,
        last,
    }) = &args.command
    {
        app.list_prices(date, currency, last);
    }
    // prune

    if let Some(Commands::Prune { symbol }) = &args.command {
        app.prune(symbol);
    }
}
