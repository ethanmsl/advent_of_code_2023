//! Library code for Part 1 of Day 07 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay07;
use crate::lexer::Tokens;
use anyhow::Result;
use derive_more::Constructor;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tracing::{event, Level};
// use miette::Result;

/// Card Types
/// note: derived order is asscending from top to bottom as written.
///       (e.g. here, c2 < c3 < c4 < ... < cA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        cT,
        cJ,
        cQ,
        cK,
        cA,
}

/// Hand Types
/// note: derived order is asscending from top to bottom as written.
///       (e.g. here, c2 < c3 < c4 < ... < cA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HType {
        h____1,
        h___22,
        h_2222,
        h__333,
        h22333,
        h_4444,
        h55555,
}

/// Hand of specific cars, with htype and a bid.
/// (ranking not specified, expected to be inferred from context)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hand {
        cards: [Card; 5],
        htype: HType,
        bid: u64,
}

// #[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay07> {
        event!(Level::INFO, "Hiii. from  day-07 Part1! :)");
        let tokens = Tokens::lexer(input);
        for tok in tokens.spanned() {
                event!(Level::WARN, "\n{:?}", tok);
        }
        todo!("day 07 - Part 1");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        32T3K 765
                        T55J5 684
                        KK677 28
                        KTJJT 220
                        QQQJA 483
                "};
                let expected = 6440;
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
