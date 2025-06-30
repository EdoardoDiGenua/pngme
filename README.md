# PNGme Implementation in Rust

This is my personal implementation of the **PNGme** project, a command-line utility to encode and decode custom ancillary chunks into PNG images.

## 📘 About the Project

This project is based on the excellent book [PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/introduction.html) by [jrdngr](https://github.com/jrdngr). It’s a great resource for learning Rust by building a real-world tool with a strong focus on encoding/decoding binary data and understanding the PNG file format.

## 🔧 Features

- Encode custom chunks into PNG files
- Decode and read custom ancillary chunks
- Remove specific chunks
- Print all custom chunks
- Built with idiomatic Rust and safe code practices

## 🚀 Usage

This project uses `cargo`:

```sh
# Build the project
cargo build

# Run with sample arguments
cargo run -- encode <PNG_PATH> <CHUNK_TYPE> <MESSAGE> <OUTPUT_PATH>
cargo run -- decode <PNG_PATH> <CHUNK_TYPE>
cargo run -- remove <PNG_PATH> <CHUNK_TYPE>
cargo run -- print <PNG_PATH>
