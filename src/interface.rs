use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="Price Database")]
#[command(author="Alen Šiljak", version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    // Add

    // Cfg

    #[command(about="Download prices")]
    Dl {
        #[arg(short='x', long)]
        exchange: Option<String>,

        #[arg(short, long)]
        symbol: Option<String>,
        
        #[arg(short, long)]
        agent: Option<String>,

        #[arg(short, long)]
        currency: Option<String>,
    },

    // todo: export

    // todo: import

    // todo: last

    // todo: list

    #[command(about="Prune old prices, leaving just the latest")]
    Prune {
        #[arg(short, long)]
        symbol: Option<String>
    },

    // securities
}