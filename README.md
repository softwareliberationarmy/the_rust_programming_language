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

## Data Types

### Integer

* numeric, no fractional value
* signed and unsigned; 8, 16, 32, 64, and 128-bit
* default is i32
* `isize` and `usize` are signed and unsigned values that are 32 or 64 bit, depending on the computer architecture
* overflow
  * in debug mode, program panics and exits if overflow encountered
  * in release mode, no error, value wraps around
  * specific methods for wrapping (wraps), checked (returns None), overflowing (val + bool), and saturating (stays at min or max)
* int / int truncates towards zero (e.g. -5 / 3 returns -1)

### Floating Point

* f32 and f64, default is f64
* always signed

### Booleans

* true or false, stored as one byte

### Characters

* 4 bytes long, represents a Unicode scalar value
* accented characters, emoji, asian characters can all be stored in character
