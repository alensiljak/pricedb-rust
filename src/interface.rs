/**
 * CLI interface
 */
#[derive(clap::Parser, Debug)]
#[command(name = "Price Database")]
#[command(author, version, about, long_about = None)]  // these are loaded from Cargo.toml
#[command(arg_required_else_help=true)]
#[command(help_template=r#"{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    // Add

    // Cfg
    // #[command(subcommand)]
    #[command(about = "Configuration")]
    #[command(arg_required_else_help(true))]
    // #[command(flatten)]
    #[clap(subcommand)]
    Config(ConfigCmd),
    // {
    //     pub(crate) Show
    // },

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

#[derive(clap::Subcommand, Debug)]
pub(super) enum ConfigCmd {
    /// Displays the current configuration
    Show,
}