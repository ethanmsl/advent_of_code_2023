//! Parsing library code for Day 08 of Advent of Code 2023.
#![allow(warnings)]

use derive_more::Unwrap;
use logos::{Lexer, Logos};

// static RE_A_TO_B: Lazy<Regex> =
//         Lazy::new(|| Regex::new(r"(?<input>\w+)-to-(?<output>\w+) map:").unwrap());
// static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
// let dists: Vec<_> = RE_NUM
//         .find_iter(line)
//         .map(|m| m.as_str().parse::<i64>().expect("parse failure"))
//         .collect();
// )
// let inp = caps
//         .name("input")
//         .expect("invalid 'input' map header")
//         .as_str()
//         .to_string();

/// Token Types for Day 07 Input
#[derive(Logos, Unwrap, Debug, PartialEq)]
pub enum Token {
        #[regex(r"[ \n]+", logos::skip)]
        Ignored,

        #[regex(r"[AKQJT98765432]{5}", to_arr)]
        ProtoHand([Card; 5]),

        #[regex(r"[0-9]+", |lex| lex.slice().parse::<u64>().expect("parse failure"))]
        Bid(u64),
}
