//! Parsing library code for Part2 of Day 08 of Advent of Code 2023.
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

use derive_more::Constructor;
use nalgebra::DMatrix;
use once_cell::sync::Lazy;
use path_input::*;
use regex::bytes;
use std::collections::{HashMap, HashSet};
use tracing::{event, Level};

pub fn process_input(
        input: &str,
) -> (
        Vec<Direction>,
        (DMatrix<u8>, DMatrix<u8>),
        (Vec<usize>, Vec<usize>),
) {
        let mut input_lines = input.lines();
        let first_line = input_lines
                .next()
                .expect("Input should have at least one line");
        let directions = path_input::line_to_directions(first_line.as_bytes());
        let component_lines = input_lines
                .skip(1)
                .map(|line| line.as_bytes())
                .collect::<Vec<_>>();
        let ((l_mat, r_mat), (start_idxs, solution_idxs)) =
                graph_components::process_components(component_lines);
        (directions, (l_mat, r_mat), (start_idxs, solution_idxs))
}

pub mod graph_components {
        use super::*;
        // NOTE: we're expanding the regex only to accomodate the test case
        const GRAPH_COMPONENT: &str = r"(?x)
                (?<inp>[A-Z0-9]{3})       ## 3 capital char input node
                \s = \s \(             # ' = ('

                (?<l_out>[A-Z0-9]{3})    ## 3 capital char leftward output node
                , \s                  # ', '

                (?<r_out>[A-Z0-9]{3})    ## 3 capital char leftward output node
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

        /// Rather messy construction of a couple Graph Matrices.
        pub fn process_components(
                input_lines: Vec<&[u8]>,
        ) -> ((DMatrix<u8>, DMatrix<u8>), (Vec<usize>, Vec<usize>)) {
                event!(Level::DEBUG, "Parsing Graph Components");
                let components: Vec<RawGraphComponent> = input_lines
                        .into_iter()
                        .map(parse_raw_graph_component)
                        .collect();

                // Collect input and output nodes separately
                let mut input_nodes = HashSet::new();
                let mut output_nodes = HashSet::new();
                for comp in &components {
                        input_nodes.insert(comp.input);
                        output_nodes.insert(comp.left_output);
                        output_nodes.insert(comp.right_output);
                }
                let start_nodes = input_nodes.iter().filter(|node| node[2] == b'A');
                let solution_nodes = input_nodes.iter().filter(|node| node[2] == b'Z');

                // Combine and sort nodes for matrix indices
                let mut nodes: Vec<_> = input_nodes.union(&output_nodes).cloned().collect();
                nodes.sort_unstable(); // Sort nodes; adjust sorting criteria as needed

                event!(
                        Level::DEBUG,
                        "Components: {}, Input: {}, Output: {}",
                        components.len(),
                        input_nodes.len(),
                        output_nodes.len()
                );

                // Check extent to which input and output nodes differ
                // (Neither needs to be included wholly in the other.  But I'm curious.)
                #[cfg(debug_assertions)]
                {
                        let diff_input_not_output: HashSet<_> =
                                input_nodes.difference(&output_nodes).collect();
                        let diff_output_not_input: HashSet<_> =
                                output_nodes.difference(&input_nodes).collect();
                        event!(
                                Level::DEBUG,
                                "Diff: Input - Output: {}, Output - Input: {}",
                                diff_input_not_output.len(),
                                diff_output_not_input.len()
                        );

                        event!(
                                Level::DEBUG,
                                "first is: {:?}, last is: {:?}, comp.len(): {}",
                                std::str::from_utf8(nodes.first().unwrap()).unwrap(),
                                std::str::from_utf8(nodes.last().unwrap()).unwrap(),
                                components.len(),
                        );
                }

                // Map nodes to indices
                let node_indices: HashMap<_, _> = nodes
                        .into_iter()
                        .enumerate()
                        .map(|(i, node)| (node, i))
                        .collect();

                // Initialize and populate matrices...
                let size = node_indices.len();
                let mut left_matrix = DMatrix::from_element(size, size, 0);
                let mut right_matrix = DMatrix::from_element(size, size, 0);

                // Populate matrices
                for comp in components {
                        // NOTE: unlike `parser1` we are not short-circuiting the "ZZZ" path here.
                        let input_idx = node_indices[&comp.input];
                        let left_idx = node_indices[&comp.left_output];
                        let right_idx = node_indices[&comp.right_output];

                        left_matrix[(left_idx, input_idx)] = 1;
                        right_matrix[(right_idx, input_idx)] = 1;
                }

                let start_idxs: Vec<usize> = start_nodes
                        .into_iter()
                        .map(|node| {
                                *node_indices
                                        .get(node)
                                        .expect("start nodes should be mapped")
                        })
                        .collect();
                let solution_idxs: Vec<usize> = solution_nodes
                        .into_iter()
                        .map(|node| {
                                *node_indices
                                        .get(node)
                                        .expect("solution nodes should be mapped")
                        })
                        .collect();

                ((left_matrix, right_matrix), (start_idxs, solution_idxs))
        }

        // PERF: Sparse Matrices
        //
        // let mut input_idxs = Vec::new();
        // let mut left_idxs = Vec::new();
        // let mut right_idxs = Vec::new();
        //
        // components.iter().for_each(|c| {
        //     input_idxs.push(node_indices[&c.input]);
        //     left_idxs.push(node_indices[&c.left_output]();
        //     right_idxs.push(node_indices[&c.right_output]);
        // });
        //
        // let left_matrix = CsrMatrix::from_iterator(
        //         Dim::from_usize(size),
        //         Dim::from_usize(size),
        //         left_idx
        //                 .into_iter()
        //                 .zip(input_idx.into_iter())
        //                 .map(|(r, c)| (r, c, true)),
        // );
        // let right_matrix = CsrMatrix::from_iterator(
        //         Dim::from_usize(size),
        //         Dim::from_usize(size),
        //         right_idx
        //                 .into_iter()
        //                 .zip(input_idx.into_iter())
        //                 .map(|(r, c)| (r, c, true)),
        // );
}

pub mod path_input {
        use super::*;
        const INPUT: &str = r"^(?<path>[LR]+)$";

        /// Two States of Direction
        /// (Each corresponding to a different Graph Matrix)
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Direction {
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
        pub fn line_to_directions(hay: &[u8]) -> Vec<Direction> {
                static RE_INPUT: Lazy<bytes::Regex> =
                        Lazy::new(|| bytes::Regex::new(INPUT).unwrap());
                let (_, [path]) = RE_INPUT
                        .captures(hay)
                        .map(|caps| caps.extract())
                        .expect("Regex Extraction Failure.");
                path.iter().map(Direction::byte_to_dir).collect()
        }
}
