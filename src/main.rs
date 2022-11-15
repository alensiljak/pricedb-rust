use clap::{Parser};

mod interface;

use interface::Commands;
use tracing::{debug, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name="Price Database")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


fn main() {
    initialize_logging();

    let cli = Cli::parse();

    debug!("Command: {:?}", cli.command);

    match &cli.command {
        Some(Commands::Dl { currency }) => download(currency),
        _ => println!("No command issued"),
    }
}

fn download(currency_option: &Option<String>) {
    trace!("In download...");

    match currency_option {
        Some(currency) => debug!("currency option: {}", currency),
        None => println!("no currency")
    }

}

fn initialize_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

}