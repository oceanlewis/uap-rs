# User Agent Parser

This crate is an implementation of a User Agent Parser, similar to those found as part of the [UA-Parser Community](https://github.com/ua-parser). It tries to remain as consistent with the other implementations as possible while remaining simple and legible.

## Getting Started

Every UA Parser implementation depends on the same `regexes.yaml` file, which is used to create the parser. You can find this file [here](https://github.com/ua-parser/uap-core) or by initializing the submodule of this repo if you have it cloned.

To get to the docs, clone the repo and run `cargo doc --open` to build the documentation

## Testing Locally

- `git submodule update --init` to get started
- `cargo test`
- `cargo test -- --nocapture` for the full results

## Performance and Benchmarking
`cargo bench` will run a criterion benchmark suite.

To see memory usage of the compiled regex list you can run the examples with a tool that tracks memory usage.

Example (on MacOS):
```
/usr/bin/time -l cargo run --examples full_parser
```