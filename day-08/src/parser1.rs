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

use derive_more::Constructor;
use once_cell::sync::Lazy;
use regex::bytes;

pub mod graph_components {
        use super::*;
        const GRAPH_COMPONENT: &str = r"(?x)
                (?<inp>[A-Z]{3})       ## 3 capital char input node
                \s = \s \(             # ' = ('

                (?<l_out>[A-Z]{3})    ## 3 capital char leftward output node
                , \s                  # ', '

                (?<r_out>[A-Z]{3})    ## 3 capital char leftward output node
                \)                    # ')'
        "; // note: `(?x)` enables "verbose mode" for regex string, where whitespace is ignored

        /// Raw Graph Component Data read from a Byte-String
        #[derive(Constructor, Debug, Clone, Copy, PartialEq, Eq)]
        struct RawGraphComponent {
                input: [u8; 3],
                left_output: [u8; 3],
                right_output: [u8; 3],
        }

        /// Parse a RawGraphComponent from a Byte-String
        /// Using Regex Capture groups
        fn parse_raw_graph_component(hay: &[u8]) -> RawGraphComponent {
                static RE_GRAPH_COMP: Lazy<bytes::Regex> =
                        Lazy::new(|| bytes::Regex::new(GRAPH_COMPONENT).unwrap());

                let (_, [inp, l_out, r_out]) = RE_GRAPH_COMP
                        .captures(hay)
                        .map(|caps| caps.extract())
                        .expect("Regex Extraction Failure.");

                RawGraphComponent {
                        input: inp.try_into().expect("invalid input node"),
                        left_output: l_out.try_into().expect("invalid input node"),
                        right_output: r_out.try_into().expect("invalid input node"),
                }
        }

        // TODO:
        // Collect all RawGraph Components.
        // Extract all Nodes into Vec.  Use Vec to defined Node indices and define Graph Matrix.
        // - Verify Input and Output Nodes are equivalent sets.
        // - Verify AAA & ZZZ positions are as expected (0, nodes.len()-1)
        // - Create two Graph Matrices (one for each direction)
        // NOTE: AAA -> 0 & ZZZ -> nodes.len()-1; so we shouldn't need to search by Node name.
}

pub mod path_input {
        use super::*;
        const INPUT: &str = r"^(?<path>[LR]+)$";

        /// Two States of Direction
        /// (Each corresponding to a different Graph Matrix)
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Direction {
                Left,
                Right,
        }
        impl Direction {
                /// Char to Dir
                /// Should this be its own function? :shrug:, lol
                fn byte_to_dir(c: &u8) -> Direction {
                        match c {
                                b'L' => Direction::Left,
                                b'R' => Direction::Right,
                                _ => panic!("invalid direction char"),
                        }
                }
        }

        /// Take a string with L*R* and return a vector of Directions
        /// (Originally written to take the first line of problem input.)
        fn line_to_directions(hay: &[u8]) -> Vec<Direction> {
                static RE_INPUT: Lazy<bytes::Regex> =
                        Lazy::new(|| bytes::Regex::new(INPUT).unwrap());
                let (_, [path]) = RE_INPUT
                        .captures(hay)
                        .map(|caps| caps.extract())
                        .expect("Regex Extraction Failure.");
                path.iter().map(Direction::byte_to_dir).collect()
        }
}
