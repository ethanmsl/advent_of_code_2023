//! Library code for Part 1 of Day 04 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocErrorDay04;
use derive_more::{AsMut, AsRef, Constructor, IntoIterator};
use miette::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::iter::FromIterator;
use tracing::info;

// // NOTE: these regex are overlapping
// // If one decides to pick of elements of athe line for PERF
// // then the borders will need to change
// const CARD_NUM: &str = r"Card (\d+):"; // Card 1num:
// const LEFT_NUMS: &str = r": (\d+) (\d+) (\d+) (\d+) (\d+) |"; // : 5nums |
// const RIGHT_NUMS: &str = r"| (\d+) (\d+) (\d+) (\d+) (\d+) (\d+) (\d+) (\d+)$"; // | 5 nums$

// PERF:: for simple gobling up numbers in a row
const NUM: &str = r"(\d+)";

#[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay04> {
        info!("Hiii. from  day-04 Part1! :)");
        let pile: CardPile = input.lines().collect();

        info!("------------pile: {:#?}", pile);
        todo!()
}

/// Represents a single scratch card.

#[derive(Constructor, Debug, PartialEq, Eq)]
struct ScratchCard {
        id: u64,
        wins_arr: [u64; 5],
        haves_arr: [u64; 8],
}

impl ScratchCard {
        /// NOTE: the multiple refernces to splits is error prone
        fn from_str(line: &str) -> Option<Self> {
                // static RE_CARD: Lazy<Regex> = Lazy::new(|| Regex::new(CARD_NUM).unwrap());
                // static RE_LEFT: Lazy<Regex> = Lazy::new(|| Regex::new(LEFT_NUMS).unwrap());
                // static RE_RIGHT: Lazy<Regex> = Lazy::new(|| Regex::new(RIGHT_NUMS).unwrap());
                static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(NUM).unwrap());

                let ordered_nums: Vec<u64> = RE_NUM
                        .find_iter(line)
                        .map(|m| m.as_str().parse::<u64>().expect("parse failure"))
                        .collect();

                match ordered_nums.len() {
                        14 => Some(ScratchCard::new(
                                ordered_nums[0],
                                ordered_nums[1..6]
                                        .try_into()
                                        .expect("vec to array failure: A"),
                                ordered_nums[6..14]
                                        .try_into()
                                        .expect("vec to array failure: B"),
                        )),
                        _ => None,
                }
        }
}

/// Represents a pile of (scratch) cards.
/// (simple wrapper)
#[derive(Debug, PartialEq, Eq)]
struct CardPile {
        cards: Vec<ScratchCard>,
}

impl CardPile {
        fn new() -> Self {
                Self { cards: Vec::new() }
        }
}

impl<'a> FromIterator<&'a str> for CardPile {
        fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
                let mut pile = CardPile::new();

                for item in iter {
                        match ScratchCard::from_str(item) {
                                Some(card) => pile.cards.push(card),
                                None => panic!("Invalid card data on line {}", item),
                        }
                }

                pile
        }
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
                "};
                let expected = 13;
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
