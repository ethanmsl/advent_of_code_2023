//! Library code for Part 1 of Day 08 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay08;
// use anyhow::Result;
use derive_more::Constructor;
use logos::{Lexer, Logos};
use miette::Result;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{event, Level};

/// Time for a repeatable path to find a solution.
/// Binary regex's minimum expansion size if given the finite automata.
///
/// ## Solution Styles:
/// - Linear Alrebgra even-divisor + remainder approach
///   - Problem is non-linear, but has a nice graph representation.
///   - We can just solve all steps until repetition.  And then calculate # of reps from the last
///   step and work out remainder.  (Sparse, binary/one-hot matrices.)
/// - Direct Automaton run along circular string.
///   - We can construct a FSM to make our automaton and run along string.
///   - Naturally pairs with problem, though it's effectively brute forcing with speed.
/// - Collapse graph/automaton into a language.  
///   - We can ... then use the language to compute an even faster, stack, jump table enabled
///   Automaton.
///   - I don't ... think we can necessarilly use the langauge for any generally eficient
///   calculation. (relative to just running the automaton along it -- though we may be able to
///   collapse the input based on the language ... I'm not sure it's guaranteed to be faster than
///   running the automaton.)
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<i64, AocErrorDay08> {
        event!(Level::INFO, "Hiii. from  day-08 Part1! :)");
        todo!("day 08 - Part 1");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example_1() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        RL

                        AAA = (BBB, CCC)
                        BBB = (DDD, EEE)
                        CCC = (ZZZ, GGG)
                        DDD = (DDD, DDD)
                        EEE = (EEE, EEE)
                        GGG = (GGG, GGG)
                        ZZZ = (ZZZ, ZZZ)
                "};
                let expected = 2;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        #[test]
        fn test_process_example_2() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        LLR

                        AAA = (BBB, BBB)
                        BBB = (AAA, ZZZ)
                        ZZZ = (ZZZ, ZZZ)
                "};
                let expected = 6;
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
