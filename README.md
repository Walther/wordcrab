# wordcrab

[![Build Status](https://github.com/Walther/wordcrab/workflows/Rust/badge.svg](https://github.com/Walther/wordcrab/actions)
[![codecov](https://codecov.io/gh/Walther/wordcrab/branch/master/graph/badge.svg)](https://codecov.io/gh/Walther/wordcrab)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)


`wordcrab` is a command-line tool for counting lines, words and characters in documents.

It is intended as a cross-platform, modern replacement for `wc`.

## Installation

- Make sure you have `rustup` and `cargo` installed: `curl https://sh.rustup.rs -sSf | sh`
- `cargo install wordcrab`
- Upgrading: `cargo install wordcrab --force`
- Uninstalling: `cargo remove wordcrab`

_TODO: installers in various OS package managers._

## Goals

- [x] MVP version that counts lines, words and characters
- [x] Publish on <crates.io>
- [ ] Publish on homebrew
- [ ] Publish on other platforms, which?
- [ ] Write a blogpost about the process of making this; as a showcase of creating and publishing a tool
- [x] Optional JSON output format
- [x] Optional YAML output format
- [x] Optional TOML output format
- [x] Concurrency with `rayon`
- [ ] Benchmark & write a blogpost about the process of benchmarking a simple tool
- [x] More options: `-l`, `-w`, `-c` similar to `wc`
- [ ] Testing
  - [x] Tiny initial tests
  - [x] Lots of tests to ensure correctness of various functions
  - [ ] Test output formatting
  - [ ] Test tricky strings
  - [ ] Full code coverage
- [x] Soft error handling; do not panic if a single file fails to be read of a long list
- [ ] Serialization error handling; currently panics. How to type `main()` and return errors?
- [ ] Async-std port, when async/await lands in stable?
- [ ] CI builds
