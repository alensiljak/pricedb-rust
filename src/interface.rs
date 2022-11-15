use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="Price Database")]
#[command(author, version, about, long_about = None)]
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
        //#[arg(short='x', long)]
        // todo: exchange: Option<String>,

        // todo: symbol
        // todo: agent

        #[arg(short, long)]
        currency: Option<String>,
    },

    // todo: export

    // todo: import

    // todo: last

    // todo: list

    #[command(about="Prune old prices, leaving just the latest")]
    Prune {
        //all: Option<>,
        symbol: Option<String>
    },

    // securities
}