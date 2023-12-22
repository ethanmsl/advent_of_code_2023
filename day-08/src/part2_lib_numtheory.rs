//! Library code for Part 2_numtheory of Day 08 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`
#![allow(warnings)]

use crate::parser2::path_input::Direction as D;
use crate::{custom_error::AocErrorDay08, parser2::process_input}; // Note: we use the same parser
                                                                  // here.
use derive_more::{Constructor, Display, Index, IntoIterator};
use itertools::Itertools;
use miette::Result;
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use std::collections::HashSet;
use tracing::{event, Level};

/// Key information about the derived transition matrices
/// Each matrix represents a progressively longer transition acording to problem directions
/// Therefore each matrix takes the same starting information.
///
/// # Example:
/// L, LR, LRL, LRLL, ...
#[derive(Debug, Constructor, Index, Display, IntoIterator)]
#[display(
        fmt = "mats: (note shown) number of matrices: {} matrix side length (square): {}",
        num_mats,
        square_side_len
)]
struct TransMats {
        #[index]
        #[into_iterator(ref)]
        mats: Vec<DMatrix<u8>>,
        num_mats: usize,
        square_side_len: usize,
}
impl TransMats {
        /// Convenience Method to get last matrix in transiton series, which represents a complete
        /// 'rotation' through the direction series.
        fn full_trans_mat(&self) -> &DMatrix<u8> {
                self.mats
                        .last()
                        .expect("transition matrices to have been calculated")
        }
}

/// Core Seed Info for the Problem
#[derive(Display, IntoIterator, Debug, Index)]
#[display(
        fmt = "start idxs: {:?}, solution_idxs: {:?},\ndirections {:?},\nl_graph: {},\nr_graph: {}]",
        start_idxs,
        solution_idxs,
        directions,
        l_graph,
        r_graph
)]
struct ProblemSpecifics {
        #[index]
        #[into_iterator(ref)]
        directions: Vec<D>,
        start_idxs: Vec<usize>,
        solution_idxs: Vec<usize>,
        l_graph: DMatrix<u8>,
        r_graph: DMatrix<u8>,
}
impl ProblemSpecifics {
        /// Convenience method to get the length of the directions vector.
        /// Length of L|R directions given, which are then to be repeated.
        /// A fundamental period of repetition.
        fn dir_len(&self) -> usize {
                self.directions.len()
        }
}

/// Naming is hard!  Struc  of critical info, tied to a start location.
/// I can't think of a good name -- but this is critical information associatted with eacy start
/// "Full_Cycle_Node_...": there are only nodes.len() number of states to be in in the graph.
/// This means that at anypoint in the directions series, we *must* repeat ourselves after
/// node.len() (or fewer) steps.
///
/// e.g.
/// if Dirs are: [L, R, L]
/// Then a Full Cycle is LRL
/// Which corresponds to the equivalent matrix multiplication
///
/// We are using a full cycle of directions as the primary point of reference.
/// From there we search for repetitions. And note:
/// - (A) "offset path": the nodes visited before a cycle began.
/// - (B) "cycle path": the nodes seris that will repeat.
///
/// Contuning:
/// - (C) we calculate when a solution output is found at every step of the way and note it's full
/// index.
///     - for [L, R, L] then if it happened on 3rd full cycle at an R it's index would be 2*3 +1
///     (zero-indexed)
/// - (D) we calculate the lcm of all cycle lengths, accout for offset + other offset, then (bleh!)
/// finsh the solution to determine when solutions all align.
#[derive(Debug, Constructor, Display)]
#[display(
        fmt = "start_id: {} full_cycle_node_offset_path: {:?} full_cycle_node_cycle_path: {:?}",
        start_id,
        full_cycle_node_offset_path,
        full_cycle_node_cycle_path
)]
struct InitBag {
        start_id: usize, // will match an element of ProblemSpecifics.start_idxs
        full_cycle_node_offset_path: Vec<usize>,
        full_cycle_node_cycle_path: Vec<usize>,
}
impl InitBag {
        /// Convenience method to get id
        fn id(&self) -> usize {
                self.start_id
        }
}

/// Hmm ... each solution can only have at most side_len number of unique full rotations
/// e.g. as soon as a full rotation hits an old input it will cycle
/// and there are only 726 or so such inputs
/// so.  we can (1) find the number of unique rotations for each.
/// then b find the ouputs for those rotations
/// From that -- we will get:
/// sol1 --------X-------X---(20) prime_decomp: 2,2,5    offsets: 8, 16
/// sol2 --X-----(8) prime_decomp: 2,2,2   offsets: 2
/// sol3 ---------------------X-------X---(33) prime_decomp: 3, 11  offsets: 21, 29
/// sol4 --XX-X----(10) prime_decomp: 2, 5  offsets: 2,3,5
///
/// From this, we can do some math and figure out when these guys overlap
/// Notably:
/// | - to warm up: sol1 & sol2 ... will repeate overlap every 2|5 times (so 40 length)
/// |     - the smallest shared prime decomp we can make from them, without breaking
/// | - So how do offsets work...  there're only 2:5 ratio before repition here; not all offsets will be valid, which would require 1:8
/// |     - quick guess, we lose the shared prime overlap of (2,2)
/// |     0       8       16          28      36 39
/// |sol1 -Y------X-------X--|-Y------X-------X--|
/// |sol2 --X--Y-|--X--Y-|--X--Y-|--X--Y-|--X--Y-|
/// |     0 2       10      18      26      34   39
/// |
/// |So, some quick maths [offset] + len * (0..reps) is the total number of offsets
/// |we're ... uhoh, adding,so our pretty prime number maths are a bit harder
/// |But we can solve this ... 2+8a  =?  8+20b --> 4a-10b =? 3  -->    a:[0..5) b:[0..2)
/// |                          2+8a  =? 16+20b --> 4a-10b =? 7  -->
/// |                                              Note: 4 & 10 share a prime factor
/// |                                                    so *ANY* solution of them must share that factor
/// |                                                    so we know there is no solution to the above two problems
/// |I've added a manufactured solution (Y)
/// |                          5+8a  =?  1+20b --> 4 =? 20b-8a --> 1 = 5b-2a @ (2,1)
/// | Sooooo, this is all very doable... but numerics solving with Rust sounds terrible...
///
/// As an exercise, check, lets see how we can visualize non-allignable components:
///
/// |   Note: to have all offsets align we'd need len1 * len2 distance
/// |        That is a full copy of a sequence for each element of it's opposite.
/// |        At an extreme (e.g. 3 & 3) there is only one copy for each element and so offsets will always be aligned.
/// |
/// |        In our case below we have 2:5 ratio. we'd need an 8:20 ratio to get arbirary alignment.
/// |        So we get 1/4 of conceivable alignments.
/// |   
/// |   Note: there are *no* repeats.
/// |          But we re-index (so to speak) into the larger sequency with an offset
/// |          of shared prime times (4, in this case)
/// |   All offsetmatches f1 + len1 a = f2 + len2 b -->
/// |       Therefore the offset difference
/// |    0      7       15      23      31     
/// |    0      7       I5      3       11
/// sol1 -------I-------I---|-------------------|  20 :. 2,2 ,5
/// sol2 -------|-------|-------|-------|-------|   8 :. 2,2 ,2
///      0      7       15  19  23      31      
///      0      0       0   3   0       0
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<usize, AocErrorDay08> {
        let (dirs, (l_mat, r_mat), (start_idxs, solution_idxs)) = process_input(input);
        event!(Level::TRACE, "directions: {:?}", dirs);
        event!(Level::INFO, "start idxs: {:?}", start_idxs);
        event!(Level::INFO, "solution_idxs: {:?}", solution_idxs);
        // [A, AB, ABC, ... AB..Z]
        let zero_to_n_transitions = dirs_to_paths(&dirs, (&l_mat, &r_mat));
        #[cfg(debug_assertions)]
        {
                for (id, mat) in zero_to_n_transitions.iter().enumerate() {
                        event!(
                                Level::TRACE,
                                "\nmat[{}]: dirs[0..=id]: {:?} \n {}",
                                id,
                                dirs[0..=id].iter().collect::<Vec<_>>(),
                                mat,
                        );
                }
        }

        let num_nodes = l_mat.nrows();
        let directions_len = dirs.len();
        let full_transition = &zero_to_n_transitions[directions_len - 1];

        // all inpts idx that create solutions for each 0-to-n transition matrix
        let solving_idx_sets: Vec<HashSet<usize>> =
                calculate_solving_inputs(&zero_to_n_transitions, &solution_idxs);
        event!(Level::INFO, num_nodes);
        event!(Level::INFO, "solution_idxs: {:?}", solution_idxs);
        event!(Level::INFO, "solving_idx_sets: {:?}", solving_idx_sets);

        // NOTE: start idxs:    [629, 347, 85, 105, 510,  0]
        //     , solution_idxs: [725, 561, 73, 589, 291, 36]
        // 8:  AAA = (SLH, CVN)
        // 165:MGA = (PLT, NCP)
        // 211:DGA = (FTD, RGR)
        // 345:TLA = (QXP, MKM)
        // 529:RDA = (BVG, DJM)
        // 545:DPA = (MDS, SHG)
        //             --
        // 39: CTZ = (NCP, PLT)
        // 52: RXZ = (SHG, MDS)
        // 60: SLZ = (DJM, BVG)
        // 113:ZZZ = (CVN, SLH)
        // 183:BSZ = (RGR, FTD)
        // 552:KKZ = (MKM, QXP)
        // event!(
        //         Level::DEBUG,
        //         "start idxs: {:?}\n, solution_idxs: {:?}",
        //         start_idxs,
        //         solution_idxs
        // );
        todo!();

        // NEED: Time-to-Cycle for Full Transitions on Each solving input
        // (context) input cycle times: [44, 80, 72, 68, 48, 54]
        //           primes:    [2. 11 | 2... 5 | 2.. 3. | 2. 17 | 2... 3 | 2 3.. ]
        //           LCM:     403_920  (not that high!, * 726 = 293_245_920 )
        //               So all align by that point, and no more novel alignments to be found.
        // I'm suspicious of this low number -- it means that any answer we find should be availble
        // in fewer than 3e8 matrix multiplications.  Which isn't crazy.
        // Suggests a bug in earlier code if true. (As I've done quite a bit more than that.)
        // WARNING: I forgot to account for an offset before cycling starts!
        // I need time to start cycle, and then cycle
        let sol_cycles: Vec<(Vec<usize>, Vec<usize>)> = start_idxs
                .par_iter()
                .map(|idx| trans_to_cycle(idx, &full_transition))
                .collect();
        // let sol_cycle_times: Vec<usize> = sol_cycles.iter().map(|cycle| cycle.len()).collect();
        // event!(Level::DEBUG, "sol_cycles: {:?}", sol_cycles);
        // event!(Level::INFO, "sol_cycle_times: {:?}", sol_cycle_times);
        //
        // // NEED: offsets for all solutions
        // let sol_offsets: Vec<Vec<(usize, usize)>> = sol_cycles
        //         .iter()
        //         .map(|sol_cycle| {
        //                 calculate_offsets(sol_cycle, &solution_idxs, &zero_to_n_transitions)
        //         })
        //         .collect();
        // event!(Level::DEBUG, "sol_offsets: {:?}", sol_offsets);
        todo!();

        // loop through rotation outputs and see if any solve
        let mut curr_inp_vec = convert_indices_to_vector(&start_idxs, num_nodes);
        let mut curr_inp_idx_set = find_ones_indices(&curr_inp_vec);
        let mut prev_inp_vec;
        let mut found = None;
        let mut rotations = 0;
        loop {
                found = solving_idx_sets
                        .par_iter()
                        .position(|sol| curr_inp_idx_set.is_subset(&sol));
                if found.is_some() {
                        break;
                };
                prev_inp_vec = curr_inp_vec;
                curr_inp_vec = full_transition * &prev_inp_vec;
                curr_inp_idx_set = find_ones_indices(&curr_inp_vec);
                rotations += 1;
                event!(Level::DEBUG, rotations);
                event!(Level::TRACE, "current input_vec: {}", curr_inp_vec);
        }
        event!(
                Level::WARN,
                "found a solution at: {}_rots + 1+{:?}_steps",
                rotations,
                found
        );

        #[cfg(debug_assertions)]
        {
                event!(Level::INFO, "solving_inputs: {:?}", solving_idx_sets);
                event!(Level::TRACE, "full_trip_matrix: {}", full_transition);
                event!(Level::INFO, "solution_idxs: {:?}", solution_idxs);
                event!(Level::INFO, "start_idxs: {:?}", start_idxs);
        }

        Ok(found.expect("should be Some if we got here") + 1 + rotations * directions_len)
}

// start1 start2 start3
// 0t1 0t2 0t3 0t4 0tn

/// Given a list of matrices and inputs, and solutions, give the offsets corresponding to
/// solutions.
fn calculate_offsets(
        start_idxs: &Vec<usize>,
        solution_idxs: &Vec<usize>,
        matrices: &Vec<DMatrix<u8>>,
) -> Vec<(usize, usize)> {
        let num_mats = matrices.len();
        matrices.iter()
                .enumerate()
                .flat_map(|(mat_num, mat)| {
                        start_idxs
                                .iter()
                                .map(|idx| get_out_node(idx, &mat.clone()))
                                .filter(|&idx| solution_idxs.contains(&idx))
                                .map(move |sol_idx| (sol_idx, mat_num))
                })
                .collect()
}

fn get_out_node(idx: &usize, mat: &DMatrix<u8>) -> usize {
        mat.column(*idx)
                .iter()
                .position(|&x| x == 1)
                .expect("all nodes should have an output")
}
/// For any finite, binary output matrix - transitions cylces must eventually occur
/// For matrices with a *single* output node they will occur within the number of nodes present
///
/// ## Soft Assumption:
/// - 1 output node per input node
///
/// ## Hard Assumption:
/// - Binary (1|0) values
///
/// # Warning:
/// These assumptions are **NOT** enforced
fn trans_to_cycle(idx: &usize, trans_matrix: &DMatrix<u8>) -> (Vec<usize>, Vec<usize>) {
        let mut output_node = idx.clone();
        let mut nodes_visited_unq = vec![output_node];
        let theoretical_limit = trans_matrix.nrows();

        // We are searching based on assumptions above, rather than  more expensive dense matrix calcs
        loop {
                output_node = trans_matrix
                        .column(*nodes_visited_unq.last().expect("populated vector"))
                        .iter()
                        .position(|&x| x == 1)
                        .expect("all nodes should have an output");
                if let Some(cycle_pos) = nodes_visited_unq.iter().position(|&x| x == output_node) {
                        let offset_path = nodes_visited_unq[0..cycle_pos].to_vec();
                        let cycling_path = nodes_visited_unq[cycle_pos..].to_vec();
                        return (offset_path, cycling_path);
                } else if nodes_visited_unq.len() > theoretical_limit {
                        panic!("Logic error - more nodes visited than nodes available!, nodes_visited_unq: {}, theoretical_limit: {}", nodes_visited_unq.len(), theoretical_limit)
                }
                nodes_visited_unq.push(output_node)
        }
        unreachable!("Only way out of loop above should have been trough panic or return.")
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
/// # use day_08::part2_lib_numtheory::calculate_solving_inputs;
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
