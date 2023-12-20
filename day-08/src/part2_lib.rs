//! Library code for Part 2 of Day 08 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use crate::parser2::path_input::Direction as D;
use crate::{custom_error::AocErrorDay08, parser2::process_input};
use miette::Result;
use nalgebra::DMatrix;
use tracing::{event, Level};

/// Part2 is an easy adaptation from Part1.
/// We just need an two sets of indices for all elements ending with A and with Z
/// And then we do the same thing, but instead of looking for ZZZ, we look for All Z's
/// (for simplicity we may as well pre-calculate that vector)
///
/// (Incidentally, at first this was sounding like an NFA style problem, but it's not
/// , but classic NFA wouldn't have the states aware of one another.  Though, implementaiton wise,
/// one outght to be able to check all the end state still.  e.g. thinkign about BurntSushi's
/// regex-automata crate: basically using a multi pattern (with pattern offsets) I *think* you
/// could get that info out.  [though not sure])
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
                        LR

                        11A = (11B, XXX)
                        11B = (XXX, 11Z)
                        11Z = (11B, XXX)
                        22A = (22B, XXX)
                        22B = (22C, 22C)
                        22C = (22Z, 22Z)
                        22Z = (22B, 22B)
                        XXX = (XXX, XXX)
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
        //         tracing_subscriber::fmt::init();
        //         let file_input = include_str!("../input2.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
