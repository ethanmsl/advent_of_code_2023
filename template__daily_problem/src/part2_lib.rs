//! Library code for Part 2 of {{ project-name | title_case }} of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`
#![allow(warnings)]

use crate::custom_error::AocError{{project-name | upper_camel_case}};
use derive_more::Constructor;
use logos::{Lexer, Logos};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{event, Level};
use miette::Result;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<i64, AocError{{project-name | upper_camel_case}}> {
        event!(Level::INFO, "Hiii. from  {{ project-name | kebab_case }} Part2! :)");
        todo!("{{project-name | title_case | downcase}} - Part 2");
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
