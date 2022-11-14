use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    // #[arg(short, long)]
    // argument: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    dl {}
}

fn main() {
    println!("Hello, world!");

    let cli = Cli::parse();

    println!("Hello {:?}!", cli.command)
}
