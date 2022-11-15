use clap::{Parser};

mod interface;

use interface::Commands;

#[derive(Parser, Debug)]
#[command(name="Price Database")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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