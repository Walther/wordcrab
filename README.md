# wordcrab

`wordcrab` is a command-line tool for counting lines, words and characters in documents.

It is intended as a cross-platform, modern replacement for `wc`.

## Goals

- [x] MVP version that counts lines, words and characters
- [x] Publish on <crates.io>
- [ ] Publish on homebrew
- [ ] Publish on other platforms, which?
- [ ] Write a blogpost about the process of making this; as a showcase of creating and publishing a tool
- [x] Optional JSON output format
- [ ] Concurrency
- [ ] Benchmark & write a blogpost about the process of benchmarking a simple tool
- [ ] More options: `-l`, `-w`, `-c` similar to `wc`
- [ ] Proper tests to ensure correctness, especially on tricky strings
- [ ] Soft error handling; do not panic if a single file fails to be read of a long list
