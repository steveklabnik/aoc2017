# Advent of Code 2017

I'm doing Advent of Code 2017 in Rust and Webassembly! You can [try it out
in your browser](https://github.com/steveklabnik/aoc2017).

## Building

You need a quite recent nightly for this to work.

```
> cargo build --release
```

## Running the tests

The tests can't be run via the wasm target, so you need to add
your platform triple. For example, mine is:

```
> cargo test --target=x86_64-pc-windows-msvc
```