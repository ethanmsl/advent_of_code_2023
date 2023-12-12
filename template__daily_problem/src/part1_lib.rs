//! Library code for Part 1 of {{ project-name | title_case }} of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocError{{project-name | upper_camel_case}};
use derive_more::Constructor;
use miette::Result;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{debug, info, trace};

// #[tracing::instrument]
pub fn process(_input: &str) -> Result<i64, AocError{{project-name | upper_camel_case}}> {
        info!("Hiii. from  {{ project-name | kebab_case }} Part1! :)");
        todo!("{{project-name | title_case | downcase}} - Part 1");
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
        //         let file_input = include_str!("../input1.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
