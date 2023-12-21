//! Library code for Part 2 of Day 08 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`
#![allow(warnings)]

use crate::parser2::path_input::Direction as D;
use crate::{custom_error::AocErrorDay08, parser2::process_input};
use miette::Result;
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use std::collections::HashSet;
use tracing::{event, Level};

/// Part2 is an easy adaptation from Part1.
/// BUG: We just need an two sets of indices for all elements ending with A and with Z
/// And then we do the same thing, but instead of looking for ZZZ, we look for All Z's
/// (for simplicity we may as well pre-calculate that vector)
/// NOTE:That solution will not work with the original efficiencie approach.  As having had each
/// sub path hit a goal sometime in a range does not guarantee that they overlapped.
/// You could use it to ignore cases where not all paths output a solution in that rotation,
/// and that would give some perf option.  But also complicate, as short circuits will obscure
/// some paths you may need.  (You could add a single tracer row/column though.)
/// - A separate, promising solution also needs to be thrown away:
///     - If there were only one point in a cycle that a solution was entered then we could
///     calculate each individually and do some math with prime components of path lenghts to
///     calculate the correct solution.  But we'd need to calculate all the entries to solution
///     states from all possible start states (since they can move over path rotations)
///     -  This is perhaps doable, but also needs to be done at each step for each start state.
///         - Which is 726* 307 = 222_882 calcs.
///             - Mind you, that's not matrix multiplications.  We still are ceilined at  307 of
///             those.  Still.  It's a bit of an issue.
///
/// OPTIONS:
/// - Take the input values as a vector and just run through all 307 calculated vectors in parallel
/// - Alongside all the full rotation outputs along each. These are mostly vec .> mat calcs, vs mat
/// calcs (still expensive, 726^2 = 500k, vs ^3).  The parallelism is nice, but small.
/// - Naively you'd also need to check the hot indices vs a hashset
/// - PERF: add an index element that all solutions ouput 1 2. -- Then you can just check if that
/// index has a value = stat_nodes.len()
///
/// ??? - What of collapsing the start nodes?  <--NO, you can copy & collapse, but not much won.
///     - So, if S1 --> A, S2 --> B, S3 --> A we make a node S123 --> AAB :([2,1,0...0])
///     - We're *already* using u8s.  So we could track numbers.
///     - WARN: there must be *no rentrance* to that state.  As the independence algebra won't be
///     able to do that logic.  In that case the multi-weighting is okay.  BUT it comes at the cost
///     of having to deal with that junk element forever.  And we can't get rid of the other start
///     nodes as they may play roles in various continuation paths.
///
/// TODO:
/// How long till world state *must* repeat?  dir.len() * row.len() -- for a single start.
///     - which is 307 * 726 = 222_882 -- 223k look ups. (not that bad)
///     - So we can look at, say, 20K (made up) possible solution points.
///     - From that, we can inspect all inputs that lead to them. .. working back to the dir
///     rotation start.  From there we can iterate on vector multiplications of full rotation
///     matrix. To find the first full time the input state gives the a solution state.
/// - ^ ... This actually sounds practical.  Even without using sparsity. Or other matrix perfs.
/// 1) Get vector of pure solve states. Get needed entry state from each stage calculation.
/// 2) (save each one along with steps to get there )
/// 3) Iterate with start vector multiplications on the full dir rotation.
///     3.1) At each stage check if output is a **SUB**set of a solver state.
///     3.2) if it ad full-rot calcs * dirs.len() + the lookup steps for solution state you're
///       included in
/// Q) Effective searching for subsets... (worst case: 307 hashsets that we do < 726 .contains()
/// for each ... but bleh! <-- with short-circuit failling probably not too bad)
///     - PERF: we could find maximally distinguishable output elements and start search with
///     those, but that's more implementation than I want to do for this.
///
/// NOTE: OBVIOUSLY, we could just do all this with an NFA.
/// - We can construct the NFA directly from the input, convert directions to a bitstring, and
/// then just run along it on rotation.  Only accepting when all solution states hit
/// simultaneously.
/// - We can direct construct a PikeVM NFA in `regex-automata`. And we can get locations of all
/// patttern matches, so, if we combined with streaming, or just some repeat tracker, we could
/// probably very efficiently solve this.  (in *principle* it could take much longer, but the
/// efficiency and likely non-catastrophic solution mean that it would probably be a great approach)
/// - And direct constructing an NFA does sound fun.  (I want to get familiar with the
/// `regex-automata` crate.  *AND* if that got akward we could translate the AutomatonGraph into a
/// BitRegex. And use regular regex.  Heck, we could even just use a parsecombinator like Nom --
/// this would probably be a great case for it. -- It would just have to have some state store and
/// check somewhere... not sure how ergonomic the NFA version would be, but prob fine.)
///
/// NOTE: 2 -- regex-automata does *not* have a nice way of constructing machines from states.
/// I did have some fun putting together a nice algorithm for constructing Languages from Machines.
/// So I could make an automaton that way.  (And it would be fun to implement the algorithim
/// anyway.  It works nicely with a graph based representaiton of network.)
/// But even then there does not seem to be a straightforwad way to apply the exisitn gregex crate
/// to bits or arbitrary enums.  And then there's the issue of simulating a circular pull on the
/// input stream.
///
/// (Incidentally, at first this was sounding like an NFA style problem, but it's not
/// , but classic NFA wouldn't have the states aware of one another.  Though, implementaiton wise,
/// one outght to be able to check all the end state still.  e.g. thinkign about BurntSushi's
/// regex-automata crate: basically using a multi pattern (with pattern offsets) I *think* you
/// could get that info out.  [though not sure])
///
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<usize, AocErrorDay08> {
        let (dirs, (l_mat, r_mat), (start_idxs, solution_idxs)) = process_input(input);
        // [A, AB, ABC, ... AB..Z]
        let start_to_x_trips = dirs_to_paths(&dirs, (&l_mat, &r_mat));

        let mat_side_len = l_mat.nrows();
        let fp_len = dirs.len();
        let full_trip = &start_to_x_trips[fp_len - 1];

        let solving_inputs: Vec<HashSet<usize>> =
                calculate_solving_inputs(&start_to_x_trips, &solution_idxs);

        // loop through rotation outputs and see if any solve
        let mut current_invec = convert_indices_to_vector(&start_idxs, mat_side_len);
        let mut current_input = find_ones_indices(&current_invec);
        let mut previous_invec;
        let mut found = None;
        let mut rotations = 0;
        loop {
                found = solving_inputs
                        .par_iter()
                        .position(|sol| current_input.is_subset(&sol));
                if found.is_some() {
                        break;
                };
                previous_invec = current_invec;
                current_invec = full_trip * &previous_invec;
                current_input = find_ones_indices(&current_invec);
                rotations += 1;
                event!(Level::DEBUG, rotations);
                event!(Level::TRACE, "current input_vec: {}", current_invec);
        }
        event!(
                Level::WARN,
                "found a solution at: {}_rots + 1+{:?}_steps",
                rotations,
                found
        );

        #[cfg(debug_assertions)]
        {
                event!(Level::INFO, "solving_inputs: {:?}", solving_inputs);
                event!(Level::TRACE, "full_trip_matrix: {}", full_trip);
                event!(Level::INFO, "solution_idxs: {:?}", solution_idxs);
                event!(Level::INFO, "start_idxs: {:?}", start_idxs);
        }

        Ok(found.expect("should be Some if we got here") + 1 + rotations * fp_len)
}

/// Finds the indices of all elements in a vector that are equal to 1.
fn find_ones_indices(vector: &DVector<u8>) -> HashSet<usize> {
        vector.iter()
                .enumerate()
                .filter_map(|(idx, &val)| if val == 1 { Some(idx) } else { None })
                .collect()
}

/// Converts a list of indices into a vector of 0s and 1s.
/// 1s are placed at the indices specified in `solution_idxs`.
fn convert_indices_to_vector(idxs: &[usize], max_size: usize) -> DVector<u8> {
        let mut vector = DVector::from_element(max_size, 0u8);

        for &idx in idxs {
                if idx < max_size {
                        vector[idx] = 1;
                }
        }

        vector
}

/// Calculates the inputs that will generate the entire set of solutions for a group of matrices.
///
/// # Warning:
/// This function assumes:
/// 1. Matrices are square.
/// 2. Matrices are 1,0 matrices.
/// 3. Matrices have a *single* 1 per column. (i.e. a single path from each node.)
///
/// # Examples
///
/// ```
/// # use nalgebra::DMatrix;
/// # use std::collections::HashSet;
/// # use day_08::part2_lib::calculate_solving_inputs; // Replace `your_crate` with the name of your crate
/// let matrices = vec![
///     DMatrix::from_row_slice(3, 3, &[
///         0, 1, 0,
///         1, 0, 0,
///         0, 0, 1,
///     ]),
///     DMatrix::from_row_slice(3, 3, &[
///         1, 0, 0,
///         0, 1, 0,
///         0, 0, 1,
///     ]),
/// ];
/// let solution_idxs = vec![1, 2];
/// let solving_inputs = calculate_solving_inputs(&matrices, &solution_idxs);
/// let expected = vec![
///     HashSet::from([0, 2]),
///     HashSet::from([1, 2]),
/// ];
/// assert_eq!(solving_inputs, expected);
/// ```
pub fn calculate_solving_inputs(
        matrices: &[DMatrix<u8>],
        solution_idxs: &Vec<usize>,
) -> Vec<HashSet<usize>> {
        let solution_vector = convert_indices_to_vector(&solution_idxs, matrices[0].ncols());
        let result: Vec<HashSet<usize>> = matrices
                .iter()
                .map(|mat| {
                        let transposed_mat = mat.transpose();
                        let result = &transposed_mat * &solution_vector;

                        // Collect indices of ones
                        result.iter()
                                .enumerate()
                                .filter_map(|(idx, &val)| if val == 1 { Some(idx) } else { None })
                                .collect::<HashSet<usize>>()
                })
                .collect();
        result
}

fn dirs_to_paths(
        directions_vec: &[D],
        path_choices: (&DMatrix<u8>, &DMatrix<u8>),
) -> Vec<DMatrix<u8>> {
        let (l_mat, r_mat) = path_choices;

        directions_vec
                .iter()
                .map(|dir| match dir {
                        D::Left => l_mat,
                        D::Right => r_mat,
                })
                .scan(
                        nalgebra::DMatrix::identity(l_mat.nrows(), l_mat.ncols()),
                        |acc, mat| {
                                let new_mat = mat * &*acc;
                                *acc = new_mat.clone();
                                Some(new_mat)
                        },
                )
                .collect::<Vec<_>>()
}

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
