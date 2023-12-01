//! Benchmarking code for use with **divan** crate.
//! Specifies two functions, corresponding to two parts for
//! a day of Advent of Code 2023 problems.

use {{crate_name}}::*;

fn main() {
        // Run registered benchmarks.
        divan::main();
}

#[divan::bench]
fn part1() {
        part1::process(divan::black_box(include_str!(
                "../input1.txt",
        )))
        .unwrap();
}

#[divan::bench]
fn part2() {
        part2::process(divan::black_box(include_str!(
                "../input2.txt",
        )))
        .unwrap();
}
