//! Benchmarking code for use with **divan** crate.
//! Specifies two functions, corresponding to two parts of the
//! Day 01 of Advent of Code 2023 problem.

use day_02::*;

fn main() {
        // Run registered benchmarks.
        divan::main();
}

#[divan::bench]
fn part1() {
        part1_lib::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
        part2_lib::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}
