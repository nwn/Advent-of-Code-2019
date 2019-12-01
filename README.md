## Advent of Code 2019

This repo contains my solutions to the
[2019 Advent of Code](https://adventofcode.com/2019) problem set.

The solutions are written in Rust, and organized such that each daily problem
has a specific branch associated with it of the form `dayXX`. Each day has two
puzzles, the solutions to which are found in the corresponding branch's
`src/XXa.rs` and `src/XXb.rs` files respectively, and invoked with `cargo run
--bin XXa` or `cargo run --bin XXb`. Puzzle input is read from stdin, with my
account-specific input located in the `input` directory.
