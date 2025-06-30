use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::{ChunkNotFound, Png};
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = png_from_file(args.png())?;
    let chunk_type = ChunkType::from_str(args.chunk_type())?;
    let chunk = Chunk::new(chunk_type, args.secret_message().as_bytes().to_vec());
    png.append_chunk(chunk);

    let mut result = File::create("result.png")?;
    result.write_all(&png.as_bytes()[..])?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = png_from_file(args.png())?;
    let result = png
        .chunk_by_type(args.chunk_type())
        .ok_or_else(|| Box::new(ChunkNotFound))?;
    println!("{}", result.data_as_string()?);
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = png_from_file(args.png())?;
    png.remove_first_chunk(args.chunk_type())?;
    let mut result = File::create("result.png")?;
    result.write_all(&png.as_bytes()[..])?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = png_from_file(args.png())?;
    println!("{}", png);
    Ok(())
}

fn png_from_file(png_path: &PathBuf) -> Result<Png> {
    let mut file = File::open(png_path)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    Png::try_from(&data[..])
}
