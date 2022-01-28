# Rust Exercises

This repository is a collection of various coding execises with either in-progress or completed solutions written in [Rust](https://www.rust-lang.org/).

# Setup

## Rust Toolchain

Recommended method for installing the rust toolchain is with [Rust toolchain installer - rustup](https://rustup.rs/). There are shell commands and release downloads available for installation on their website.

Recommended extensions to the toolchain:

```sh
# provides `cargo add <package>` and `cargo remove <package>`
# to add external modules (crates) from https://crates.io/
# otherwise requires manual editing of Cargo.toml
cargo install cargo-edit

# provides ability to run commands on project file changes, respects .gitignore
# `cargo watch -x run` will auto-rebuild and run the application
# `cargo watch -s <expr>` will run shell commands
cargo install cargo-watch

# optional for learning urposes
# can be used to see precompiled source before final compilation
cargo install cargo-expand
```

# Usage

## Tests

```sh
# run unit tests against the unoptimized debug build
cargo test

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x test

# run unit tests against the optimized release build
cargo test --release

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test --release"
```

```sh
# run unit tests with "example" in the test function name
# against the unoptimized debug build
cargo test example

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test example"

# run unit tests against only one specific exercise as an unoptimized debug build
cargo test -p regular-expression-matching

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test -p regular-expression-matching"

# run unit tests with "example" in the test function name
# against only one specific exercise as an unoptimized debug build
cargo test -p regular-expression-matching example

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test -p regular-expression-matching example"
```

## Benchmarks

```sh
# run performance benchmarking tests for all exercises across all days
cargo bench

# run performance benchmarking tests for all exercises
# for only the regular-expression-matching exercise benchmarks
cargo bench -p regular-expression-matching
```

Note that `criterion`, the benchmarking library, can generate HTML reports with detailed graphs if `gnuplot` is installed. On Debian systems this would look like `apt install gnuplot`. This is not required to use this repository, but is fun to look at.

These html reports are generated under the folder `target/criterion` from the root of the repository. One easy way to host the files for viewing is with the following utility via cargo not listed earlier in the readme.

```sh
# provides a quick way to serve files for viewing in a web browser
cargo install https

# launch the server for the criterion benchmarking report
http ./target/criterion
```

From there you can navigate to [http://localhost:8000/report](http://localhost:8000/report).