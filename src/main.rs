use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Add
    // Cfg
    Dl {
        // exchange, -x
        // symbol
        // agent
        #[arg(short, long)]
        currency: Option<String>,
    },
    // export
    // import
    // last
    // list
    Prune {
        //all: Option<>,
        symbol: Option<String>
    },
    // securities
}

fn main() {
    let cli = Cli::parse();

    println!("Hello {:?}!", cli.command);
    match cli.command {
        Some(dl) => print!("yo"),
        _ => print!("No command issued"),
    }
}
