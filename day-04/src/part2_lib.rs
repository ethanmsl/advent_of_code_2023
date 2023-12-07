//! Library code for Part 1 of Day 04 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocErrorDay04;
use derive_more::{Constructor, IntoIterator};
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
// const RIGHT_NUMS: &str = r"| (\d+) (\d+) (\d+) (\d+) (\d+) (\d+) (\d+) (\d+)$"; // | 8 nums$

// PERF:: for simple gobling up numbers in a row
const NUM: &str = r"(\d+)";

// #[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay04> {
        info!("Hiii. from  day-04 Part1! :)");
        let mut pile: CardPile = input.lines().collect();

        // add copy value to each card (mutable)
        // iteratively (top down) get cards id & # wins
        // increment copy value of each card below
        // continue on
        // copy value update is equal to a cards own copy value
        // as you can only be modified by cards a bove you a single pass iteration should be fine
        // (kindness: no need to deal with bottom of pile boundry)
        let pile_size = pile.cards().len();
        for card_id in 1..=pile_size {
                pile.update_copies(card_id as u64);
        }

        // info!("pile: {:?}", pile);
        println!("fsdfsdfasdfadsfdsaf------------------");

        Ok(pile.cards().into_iter().map(|c| c.copies()).sum())
}

/// Represents a single scratch card.

#[derive(Constructor, Debug, PartialEq, Eq)]
struct ScratchCard {
        id: u64,
        copies: u64,
        // wins_arr: [u64; 5],
        // haves_arr: [u64; 8],
        wins_arr: [u64; 10],
        haves_arr: [u64; 25],
}

impl ScratchCard {
        /// number of copies of a card
        /// (all cards start at 1)
        fn copies(&self) -> u64 {
                self.copies
        }

        /// Gives the number of winning nums
        /// assumes wins_arr could be a set (i.e. ignoring non-uniqueness) while haves_arr may have
        /// duplicates
        fn wining_haves_overlap(&self) -> u64 {
                self.haves_arr
                        .iter()
                        .filter(|&n| self.wins_arr.contains(n))
                        .count() as u64
        }

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
                        // 14 => Some(ScratchCard::new(
                        36 => Some(ScratchCard::new(
                                ordered_nums[0],
                                1,
                                // ordered_nums[1..6]
                                ordered_nums[1..11]
                                        .try_into()
                                        .expect("vec to array failure: A"),
                                // ordered_nums[6..14]
                                ordered_nums[11..36]
                                        .try_into()
                                        .expect("vec to array failure: B"),
                        )),
                        _ => None,
                }
        }
}

/// Represents a pile of (scratch) cards.
/// (simple wrapper)
#[derive(Debug, PartialEq, Eq, IntoIterator)]
struct CardPile(Vec<ScratchCard>);

impl CardPile {
        fn new() -> Self {
                Self(Vec::new())
        }

        /// Gives a mutable reference to the pile's cards
        fn cards(&mut self) -> &mut Vec<ScratchCard> {
                &mut self.0
        }

        fn update_copies(&mut self, card_id: u64) {
                let loc = card_id as usize - 1;
                let head_card = &self.cards()[loc];
                let head_copies = head_card.copies();

                let wins = head_card.wining_haves_overlap() as usize;
                for card in &mut self.cards()[loc + 1..=loc + wins] {
                        card.copies += head_copies;
                }
        }
}

impl<'a> FromIterator<&'a str> for CardPile {
        fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
                let mut pile = CardPile::new();

                for item in iter {
                        match ScratchCard::from_str(item) {
                                Some(card) => pile.cards().push(card),
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

        // WARNING: the structure of the data is different from the main input
        // the code is currently designed for the main input for perf reasons
        // (and I just don't feel like doing multi pass or splitting)
        #[ignore]
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
                let expected = 30;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        /// This test's expected value is to be populated after
        /// verification of solution.
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        #[test]
        fn test_process_problem_input() -> Result<()> {
                tracing_subscriber::fmt::init();

                let file_input = include_str!("../input2.txt");
                let expected = 5667240;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
