use clap::{Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    // Add

    // Cfg

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

    Prune {
        //all: Option<>,
        symbol: Option<String>
    },

    // securities
}