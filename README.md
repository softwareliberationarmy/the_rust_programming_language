# NOTES: The Rust Programming Language

## Cargo commands

### Creation

NOTE: Cargo project names cannot start with a number

* `cargo new my_project_name` - creates new dir w/ main boilerplate and Cargo.toml file
* `cargo init` - for retrofitting cargo into a non-cargo rust project. Run it in a folder with a `src` subfolder and it will create the Cargo.toml file

### Compilation

* `cargo build` - compiles the project and outputs the exe to target/debug/
* `cargo build --release` - compiles the project in release mode
* `cargo run` - compile and launch exe all in one step
* `cargo check` - dry run for build

### Dependencies

* `cargo update` - updates all dependencies using semver dependency rules (e.g. if I have a dependency version 0.8.5 specified, and there is a version 0.8.6 and a version 0.9.0, the system will upgrade to version 0.8.6)
* `cargo doc --open` generates documentation on all of the dependencies installed and opens it in the default browser
