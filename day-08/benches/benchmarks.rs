//! Benchmarking code for use with **divan** crate.
//! Specifies two functions, corresponding to two parts of the
//! Day 08 of Advent of Code 2023 problem.

use day_08::*;

fn main() {
        // Run registered benchmarks.
        divan::main();
}

#[divan::bench]
fn part1() {
        part1_lib::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

// Too Slow!
// #[divan::bench]
// fn part2_lib_rawmatrix() {
//         part2_lib_rawmatrix::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
// }

#[divan::bench]
fn part2_lib_numtheory() {
        part2_lib_numtheory::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}
