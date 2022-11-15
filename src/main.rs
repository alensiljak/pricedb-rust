use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="Price Database")]
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

fn main() {
    let cli = Cli::parse();

    println!("Debug: {:?}!", cli.command);

    match &cli.command {
        Some(Commands::Dl { currency: _ }) => download(cli.command),
        _ => println!("No command issued"),
    }
}

fn download(dl: Option<Commands>) {
    println!("{:?}", dl);
    //dl = Commands::Dl
    // if currency.is_some() {
    //     println!("download for currency {:?}", currency)
    // } else {
    //     println!("plain download")
    // }

}