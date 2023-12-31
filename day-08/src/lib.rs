//! Library code for Day 08 of Advent of Code 2023.
//! This file is principally for collecting and ordering code.
//! And the architecture of the crate will be hierarchical about `part1_lib.rs` and part2_lib.rs`.
//! (With redundancy common, and by design, between those two files.)

pub mod custom_error;

pub mod parser1;
pub mod parser2;

pub mod part1_lib;
pub mod part2_lib_numtheory;
pub mod part2_lib_rawmatrix;
