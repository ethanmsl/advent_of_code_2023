//! Library code for Part 1 of Day 08 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::parser1::path_input::{self, Direction as D};
use crate::{custom_error::AocErrorDay08, parser1::process_input};
use nalgebra::{DMatrix, DVector};
// use anyhow::Result;
use derive_more::Constructor;
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
///   - e.g.
///
///   (L) a b c d e g z
///    a
///    b  1
///    c
///    d    1   1
///    e          1
///    g            1
///    z      1       1
///
///   (R) a b c d e g z
///    a
///    b
///    c  1
///    d      1 1
///    e    1     1
///    g            1
///    z              1
///
///    Divisor = RL (reversed becuase contemporary math notation is hide bound)
///
///   (L) a b z
///    a    1
///    b  1
///    z      1
///
///   (R) a b z
///    a
///    b  1
///    z    1 1
///
///  (LL) a b z
///    a  1
///    b    1
///    z      1
///
///    Divisor = RLL (reversed becuase contemporary math notation is hide bound)
///
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
///
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<usize, AocErrorDay08> {
        event!(Level::INFO, "Hiii. from  day-08 Part1! :)");
        let (dirs, l_mat, r_mat) = process_input(input);
        event!(Level::INFO, "dirs: {:?}", dirs);
        event!(Level::TRACE, "l_mat: {}", l_mat);
        event!(Level::TRACE, "r_mat: {}", r_mat);
        let fp_len = dirs.len();

        // Basic matrix multiplication
        let trips = dirs
                .iter()
                .map(|dir| match dir {
                        D::Left => &l_mat,
                        D::Right => &r_mat,
                })
                .scan(
                        nalgebra::DMatrix::identity(l_mat.nrows(), l_mat.ncols()),
                        |acc, mat| {
                                let new_mat = mat * &*acc;
                                *acc = new_mat.clone();
                                Some(new_mat)
                        },
                )
                .collect::<Vec<_>>();
        #[cfg(debug_assertions)]
        {
                for (id, mat) in trips.iter().enumerate() {
                        event!(
                                Level::TRACE,
                                "\nmat[{}]: dirs[0..=id]: {:?} \n {}",
                                id,
                                dirs[0..=id].iter().collect::<Vec<_>>(),
                                mat,
                        );
                }
        }

        let full_trip_matrix = &trips[fp_len - 1];
        let (ub, final_start_idx) =
                get_upper_bound(full_trip_matrix, 100).expect("no upperbound solution found");
        event!(
                Level::INFO,
                "ub: {:?} * {} (length of full rotation), with final start index of: {}",
                ub,
                fp_len,
                final_start_idx,
        );
        let remainder_trips =
                get_trips_to_end(&trips, final_start_idx).expect("no remainder solution found");
        let total_trips = (ub - 1) * dirs.len() + remainder_trips + 1;
        event!(Level::INFO, "remainder_trips: {}", remainder_trips);
        event!(Level::INFO, "total_trips: {}", total_trips);

        Ok(total_trips)
}
/// Returns the upper bound on the number of complete trips to get a solution and index to start at
fn get_upper_bound(mat: &DMatrix<u8>, some_reasonable_limit: usize) -> Option<(usize, usize)> {
        let size = mat.ncols();

        // AAA is always 0
        let mut current_index = 0;

        for n in 1..=some_reasonable_limit {
                // Only one non-zero per column
                let next_index = mat.column(current_index).iter().position(|&x| x != 0)?;

                // ZZZ is always len()-1
                if next_index == size - 1 {
                        return Some((n, current_index));
                }

                current_index = next_index;
        }

        None
}

/// Returns the number of trips it takes to get to the same ZZZ index from a given start index
fn get_trips_to_end(trips: &[DMatrix<u8>], start_index: usize) -> Option<usize> {
        let size = trips.first().expect("empty matrix vector").clone().ncols();

        event!(Level::TRACE, "trips: {:?}", trips);
        event!(Level::DEBUG, "start_index: {:?}", start_index);
        for (n, mat) in trips.iter().enumerate() {
                // Only one non-zero per column
                let next_index = mat.column(start_index).iter().position(|&x| x != 0)?;

                // ZZZ is always len()-1
                if next_index == size - 1 {
                        return Some(n);
                }
        }

        // If the end is not reached, return None
        None
}
// (L) a b c d e g z
//  a
//  b  1
//  c
//  d   1    1
//  e          1
//  g            q
//  z      1

// (R) a b c d e g z
//  a
//  b
//  c  1
//  d        1
//  e    1     1
//  g
//  z              1

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
        // // #[ignore]
        // #[test]
        // fn test_process_problem_input() -> Result<()> {
        //         tracing_subscriber::fmt::init();
        //         let file_input = include_str!("../input1.txt");
        //         let expected = 0;
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
