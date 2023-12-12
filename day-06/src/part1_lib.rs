//! Library code for Part 1 of Day 06 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay06;
use derive_more::Constructor;
use miette::Result;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{debug, info, trace};

// #[tracing::instrument]
/// Return all integer pairs less oblong than those of 'record' labelled by "Distance".
///
/// ## Example
/// Time: 7, Distance: 9
/// Find  x * (7 - x) == 9
/// get int_ceil(x)
/// Solution = Time -2x (where x is the edges we wouldn't compete in)
///
/// ## Approaches:
/// - We coudl work with floating points and do calculus.
/// - We can brute force explore integer pairs until we beat it.
///   - Biggest input number is 94, and max search is less than half of that. (if we assume all are
///   winnable)
/// - We can do some algebra and just solve a quadratic equation. (e.g. -9 + 7x x^2 == 0)
///
/// Given that we're a bit behidn brute force is probably simplest given the nature of the problem.
/// Or we could just look up the quatdratic solution formula -- lol :
///   - `x = (-b +/- sqrt(b^2 - 4ac)) / 2a`
///
pub fn process(_input: &str) -> Result<i64, AocErrorDay06> {
        info!("Hiii. from  day-06 Part1! :)");
        todo!("day 06 - Part 1");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        Time:      7  15   30
                        Distance:  9  40  200
                "};
                let expected = 288;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        // /// This test's expected value is to be populated after
        // /// verification of solution.
        // /// (useful for future refactors and perfs)
        // /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        // #[test]
        // fn test_process_problem_input() -> Result<()> {
        //         let file_input = include_str!("../input1.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
