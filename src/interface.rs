use clap::{Command, Parser, Subcommand};

#[derive(Parser, Debug)]
// #[derive(Clap)]
#[command(name = "Price Database")]
#[command(author="Alen Šiljak", version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    // Add

    // Cfg
    #[command(about = "Configuration")]
    Config {
        // #[command(subcommand)]
        // show {},
    },

    #[command(about = "Download prices")]
    Dl {
        #[arg(short = 'x', long)]
        exchange: Option<String>,

        #[arg(short, long)]
        symbol: Option<String>,

        #[arg(short, long)]
        agent: Option<String>,

        #[arg(short, long)]
        currency: Option<String>,
    },

    // todo: export
    #[command(about = "Export prices in ledger format")]
    Export {},

    // todo: import

    // todo: last

    // todo: list
    #[command(about = "Prune old prices, leaving just the latest")]
    Prune {
        #[arg(short, long)]
        symbol: Option<String>,
    },
    // securities
}

/// Example for how to configure the CLI.
pub fn configure_cli() -> Command {
    let cmd = clap::command!()
        .arg_required_else_help(true)
        //.arg(clap::arg!(--two <VALUE>).required(true))
        //.arg(clap::arg!(--one <VALUE>).required(true));
        .subcommand(Command::new("config"))
        .subcommand(Command::new("dl"))
        .subcommand(get_export_cmd())
        .subcommand(Command::new("prune"));

    cmd

    //clap::Command::new("rule").arg_required_else_help(true);
}

fn get_export_cmd() -> Command {
    Command::new("export")
}
