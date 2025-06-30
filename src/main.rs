use args::{Cli, Commands};
use clap::Parser;
use commands::{decode, encode, print_chunks, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd() {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print_chunks(args),
    }
}
