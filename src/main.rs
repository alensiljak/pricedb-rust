/*!
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

    match &args.command {
        // config
        Some(Commands::Config(interface::ConfigCmd::Show)) => app.config_show(),

        // export
        Some(Commands::Export {}) => app.export(),

        // download
        Some(Commands::Dl {
            currency,
            agent,
            exchange,
            symbol,
        }) => {
            let filter = SecurityFilter {
                currency: currency.clone(),
                agent: agent.clone(),
                exchange: exchange.clone(),
                symbol: symbol.clone(),
            };
            app.download_prices(filter).await;
        }

        // list
        Some(Commands::List {
            date,
            currency,
            last,
        }) => {
            app.list_prices(date, currency, last);
        }

        // prune
        Some(Commands::Prune { symbol }) => {
            app.prune(symbol);
        }

        Some(Commands::Quote {
            symbols_file,
            price_file,
            currency,
            agent,
            exchange,
            symbol,
        }) => {
            let filter = SecurityFilter {
                currency: currency.clone(),
                agent: agent.clone(),
                exchange: exchange.clone(),
                symbol: symbol.clone(),
            };

            app.dl_quote(symbols_file, price_file, filter);
        }

        None => println!("No command issued."),
    }
}
