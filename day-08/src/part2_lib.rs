//! Library code for Part 2 of Day 08 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay08;
// use anyhow::Result;
use derive_more::Constructor;
use miette::Result;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{event, Level};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<i64, AocErrorDay08> {
        event!(Level::INFO, "Hiii. from  day-08 Part2! :)");
        todo!("day 08 - Part 2");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        // #[test]
        // fn test_process_example() -> Result<()> {
        //         tracing_subscriber::fmt::init();
        //
        //         let input = indoc! {"
        //                 ...
        //                 ...
        //                 ...
        //         "};
        //         let expected = todo!();
        //         assert_eq!(process(input)?, expected);
        //         Ok(())
        // }

        // /// This test's expected value is to be populated after
        // /// verification of solution.
        // /// (useful for future refactors and perfs)
        // /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        // #[test]
        // fn test_process_problem_input() -> Result<()> {
        //         tracing_subscriber::fmt::init();
        //         let file_input = include_str!("../input2.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
