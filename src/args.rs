use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A tool to hide and retrieve secret messages inside PNG files
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

impl Cli {
    pub fn cmd(self) -> Commands {
        self.cmd
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encodes a message into a PNG file and saves the result
    Encode(EncodeArgs),

    /// Searches for a message hidden in a PNG file and prints the message if one is found
    Decode(DecodeArgs),

    /// Removes a chunk from a PNG file and saves the result
    Remove(RemoveArgs),

    /// Prints all of the chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the PNG file where the message will be hidden
    png: PathBuf,

    /// Chunk type (4 ASCII characters, e.g. "ruSt")
    chunk_type: String,

    /// The secret message to encode
    secret_message: String,
}

impl EncodeArgs {
    pub fn png(&self) -> &PathBuf {
        &self.png
    }

    pub fn chunk_type(&self) -> &str {
        &self.chunk_type
    }

    pub fn secret_message(&self) -> &str {
        &self.secret_message
    }
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the PNG file to decode
    png: PathBuf,

    /// Chunk type to decode (4 ASCII characters)
    chunk_type: String,
}

impl DecodeArgs {
    pub fn png(&self) -> &PathBuf {
        &self.png
    }

    pub fn chunk_type(&self) -> &str {
        &self.chunk_type
    }
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to the PNG file to remove chunk from
    png: PathBuf,

    /// Chunk type to remove (4 ASCII characters)
    chunk_type: String,
}

impl RemoveArgs {
    pub fn png(&self) -> &PathBuf {
        &self.png
    }

    pub fn chunk_type(&self) -> &str {
        &self.chunk_type
    }
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to the PNG file to print chunk types from
    png: PathBuf,
}

impl PrintArgs {
    pub fn png(&self) -> &PathBuf {
        &self.png
    }
}
