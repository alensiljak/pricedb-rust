/*
 * Price Database
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

    // todo: CLI configuration
    // let cmd = interface::configure_cli();
    // let matches = cmd.get_matches();

    let args = Cli::parse();

    log::debug!("Command: {:?}", args.command);

    if args.command.is_none() {
        // use clap::CommandFactory;
        // let mut cmd = Args::command();
        // cmd.print_help();
        // clap::App::print_help();
        println!("No command issued");
        return;
    }

    let app = pricedb::App::new();

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

    // prune

    if let Some(Commands::Prune { symbol }) = &args.command {
        app.prune(symbol);
    }
}
