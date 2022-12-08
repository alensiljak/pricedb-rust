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

    // CLI configuration
    //clap::Command::new("rule").arg_required_else_help(true);

    let cli = Cli::parse();

    log::debug!("Command: {:?}", cli.command);

    if cli.command.is_none() {
        // use clap::CommandFactory;
        // let mut cmd = Args::command();
        // cmd.print_help();
        println!("No command issued");
        return;
    }

    let app = pricedb::App::new();

    // export

    if let Some(Commands::Export {}) = &cli.command {
        app.export();
    }

    // download

    if let Some(Commands::Dl {
        currency,
        agent,
        exchange,
        symbol,
    }) = &cli.command
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

    if let Some(Commands::Prune { symbol }) = &cli.command {
        app.prune(symbol);
    }
}
