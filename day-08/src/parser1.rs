//! Parsing library code for Day 08 of Advent of Code 2023.
//!
//! Comments on parsing explored recently here:
//! (For context there was previously some Logos crate seed code.  And I expanded that while
//! exploring parsing options.)
//!
//! Spent a fair bit of time comparing Logos crate options.
//! It is an attractive crate with a number of upsides (including nominal
//! performance).  However, it uses custom rolled regex, has lacking
//! documentation on a variety of issues (particularly related to the regex
//! implementation and limitations).
//!
//! By constrat BurntSushi's regex crate is heavily used and the amount of
//! attention and their work (and writings on regex) instill a lot of faith
//! regarding correctness.  It's also a more reliable standard.
//!
//! The default regex crate is balancing a number of issues that will impact
//! performance. (e.g. compiling regex at runtime and choosing regex
//! implementations to avoid pathological compile cases)
//!
//! However, even if that were a forced cost the reliability and
//! standardization (and excellent documentation) would probably be a
//! worthwhile trade-off for most cases.  Additionally, however: BurntSushi
//! recently published a `regex-automata` crate allowing access to the
//! underlyign models.  Including dense DFA and on the fly calculated NFA
//! models.  For a known regex, compilation can be done ahead of time and
//! rolled into the crate via include_bytes! macro.
//!
//! This feels like an excellent option, with long legs should I decide to
//! improve performance that way.
// #![allow(warnings)]
#![allow(dead_code)]

use once_cell::sync::Lazy;
use regex::Regex;

static RE_INPUT: Lazy<Regex> = Lazy::new(|| Regex::new(INPUT).unwrap());
const INPUT: &str = r"[LR]+";

// note: `(?x)` enables "verbose mode" for regex string, where whitespace is ignored
static RE_GRAPH_COMP: Lazy<Regex> = Lazy::new(|| Regex::new(GRAPH_COMPONENT).unwrap());
const GRAPH_COMPONENT: &str = r"(?x)
(?<in>[A-Z]{3})           ## 3 capital char input node
\s = \s \(                # ' = ('

(?<l_out>[A-Z]{3})     ## 3 capital char leftward output node
, \s                      # ', '

(?<r_out>[A-Z]{3})    ## 3 capital char leftward output node
\)                        # ')'
";

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

/// Two States of Direction
/// (Each corresponding to a different Graph Matrix)
enum Direction {
        Left,
        Right,
}

/// Char to Dir
/// Should this be its own function? :shrug:, lol
fn char_to_dir(c: char) -> Direction {
        match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("invalid direction char"),
        }
}

//Line 1 : ^[LR]+$ Ls & Rs, in order.
//Line 3 +:
// AAA = (BBB, CCC)
// (?<in>[A-Z]{3}) = \( (?<left_out>[A-Z]{3}) , (?<right_out>[A-Z]{3}) \)
