# NOTES: The Rust Programming Language

## Cargo commands

* `cargo new my_project_name` - creates new dir w/ main boilerplate and Cargo.toml file
* `cargo init` - for retrofitting cargo into a non-cargo rust project. Run it in a folder with a `src` subfolder and it will create the Cargo.toml file
* `cargo build` - compiles the project and outputs the exe to target/debug/
* `cargo build --release` - compiles the project in release mode
* `cargo run` - compile and launch exe all in one step
* `cargo check` - dry run for build

### Cargo notes

* Cargo project names cannot start with a number
