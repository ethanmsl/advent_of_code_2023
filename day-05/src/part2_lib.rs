//! Library code for Part 2 of Day 05 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay05;
use miette::Result;
use tracing::info;

// #[tracing::instrument]
pub fn process(_input: &str) -> Result<i64, AocErrorDay05> {
        info!("Hiii. from  day-05 Part2! :)");
        todo!("day 05 - Part 2");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        ...
                        ...
                        ...
                "};
                let expected = todo!();
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
        //         let file_input = include_str!("../input2.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
