//A command-line program to play Marco Polo
use clap::Parser;
use hello_marco::marco_polo;

#[derive(Parser)]
#[clap(version = "1.0", author = "Martins", about = "A Marco Polo Game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
enum Command {
    #[clap(version ="1.1", author = "Martins")]
    Play {
        #[clap(short, long, help = "The name to respond to")]
        name: String,
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Command::Play { name }) => {
            println!("{}", marco_polo(name.as_str()));
        }
        None => {
            println!("No command provided");
        }
    }
}
