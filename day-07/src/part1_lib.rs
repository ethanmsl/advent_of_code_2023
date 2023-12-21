//! Library code for Part 1 of Day 07 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

// use crate::custom_error::AocErrorDay07;
use crate::lexer_1::{Card, Token};
use anyhow::Result;
use derive_more::Constructor;
use itertools::Itertools;
use logos::Logos;
// use once_cell::sync::Lazy;
use rayon::prelude::*;
// use regex::Regex;
use std::collections::HashMap;
use tracing::{event, Level};
// use miette::Result;

/// Hand Types
/// note: derived order is asscending from top to bottom as written.
///       (e.g. here, c2 < c3 < c4 < ... < cA)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HType {
        H____1,
        H___22,
        H_2222,
        H__333,
        H22333,
        H_4444,
        H55555,
}
impl HType {
        fn from_hand(hand: &Hand) -> Option<HType> {
                let mut fmap = HashMap::<Card, u8>::new();
                for card in hand.cards.iter() {
                        let count = fmap.entry(*card).or_insert(0);
                        *count += 1;
                }
                let mut counts: Vec<u8> = fmap.into_values().collect();
                counts.sort();
                match counts.as_slice() {
                        [_, _, _, _, _] => Some(HType::H____1),
                        [_, _, _, _] => Some(HType::H___22),
                        [_, 2, _] => Some(HType::H_2222),
                        [_, _, 3] => Some(HType::H__333),
                        [_, 3] => Some(HType::H22333),
                        [_, 4] => Some(HType::H_4444),
                        [_] => Some(HType::H55555),
                        _ => panic!("unhandalable hand: {:?}", hand),
                }
        }
}

/// Hand of specific cars, with htype and a bid.
/// (ranking not specified, expected to be inferred from context)
/// Ord is derivable with dictionary ordering from top to bottom as written.
/// NOTE: this is a bit the opposite of Enums, where lower is higher
///       here higher is higher.  ...that's quite unfortunate.
#[derive(Constructor, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hand {
        htype: Option<HType>,
        cards: [Card; 5],
        bid: u64,
}
impl Hand {
        /// Sets hand's htype if not already set.
        fn determine_htype(&mut self) {
                if self.htype.is_some() {
                        return;
                }
                self.htype = HType::from_hand(self);
        }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<u64> {
        event!(Level::INFO, "Hiii. from  day-07 Part1! :)");
        let mut hands: Vec<Hand> = Token::lexer(input)
                .spanned()
                .filter_map(|(t, _)| t.ok())
                .chunks(2)
                .into_iter()
                .map(|mut chunk| {
                        let hand = chunk.next().unwrap();
                        let bid = chunk.next().unwrap();
                        Hand::new(None, hand.unwrap_proto_hand(), bid.unwrap_bid())
                })
                .inspect(|h| event!(Level::TRACE, "Hand: {:?}", h))
                .collect();

        hands.par_iter_mut().for_each(|h| {
                (*h).determine_htype();
                event!(Level::TRACE, "Hand: {:?}", h);
        });
        event!(Level::TRACE, "Hands: {:?}", hands);
        // using stable sort just in case...
        hands.sort();
        event!(Level::DEBUG, "Hands: {:?}", hands);

        Ok(hands.par_iter()
                .enumerate()
                .map(|(id, h)| h.bid * (id as u64 + 1))
                .sum())
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

        /// This test's expected value is to be populated after
        /// verification of solution.
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input1.txt");
                let expected = 248836197;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
