use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Dl {
        #[arg(short, long)]
        currency: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    println!("Hello {:?}!", cli.command)
}
