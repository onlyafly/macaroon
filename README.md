# quivi: The Quivi Programming Language

## Set up your environment

Install Rustup:

    curl https://sh.rustup.rs -sSf | sh

Install Rust:

    ???

Set up Atom:

* Install ide-rust Atom package
* Enable format on save (see instructions in the ide-rust package)
* Install language-rust Atom package
* Install build-cargo and atom-build Atom packages
* Setup atom-build to build on each file save and to not steal focus.

Instructions for using the environment:

* Each time you save a file, it will run the test suite.

## Executing

    cargo run

## Testing

    cargo test
