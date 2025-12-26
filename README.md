# Rust Playground

A personal Rust playground for experimenting with concepts from the [Comprehensive Rust](https://google.github.io/comprehensive-rust/) course.

## Project Structure

- This repo is full of small, independent files, organized into projects (top-level directories).
- Each workspace contains it's own `src/bin/` directory, which will contain source files.
- Binaries are referred to by the name of the file in `src/bin`, without the `.rs` suffix.
- `Cargo.toml` — project manifest
- `Cargo.lock` — locked dependencies

## Running examples

Run `cargo run -p NAME_OF_PROJECT --bin NAME_OF_BINARY`

- `NAME_OF_PROJECT` is the top-level directory the file lives in
- `NAME_OF_BINARY` is the file name, without the `.rs` suffix
