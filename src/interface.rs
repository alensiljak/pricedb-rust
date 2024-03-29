/*!
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
    // Cfg
    #[command(about = "Configuration")]
    #[command(arg_required_else_help(true))]
    #[clap(subcommand)]
    Config(ConfigCmd),

    #[command(about = "Download latest prices and add to the prices text file")]
    Dl {
        #[arg(short, long)]
        price_file: Option<String>,
        #[arg(short='f', long)]
        symbols_file: Option<String>,
        // Symbol filters
        #[arg(short, long)]
        currency: Option<String>,
        #[arg(short, long)]
        agent: Option<String>,
        #[arg(short = 'x', long)]
        exchange: Option<String>,
        #[arg(short, long)]
        symbol: Option<String>,
}
}

#[derive(clap::Subcommand, Debug)]
pub(super) enum ConfigCmd {
    /// Displays the current configuration
    Show,
}