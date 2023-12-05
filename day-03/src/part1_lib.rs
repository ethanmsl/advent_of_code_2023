//! Library code for Part 1 of Day 03 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocErrorDay03;
use crate::data_types_part1::{NumberRegister, SpecialAdjacenciesRegister};
use miette::Result;
use tracing::info;

/// Return sum of values adjacent to special chars
///
/// ## Proposed Flow
/// - get number locations --> map each to number
/// - get special locations --> calculate adjacency locations
/// - comapre number locations against adjacencies
///   - you only want one of each number
///
/// ## Assumptions
/// - Numbers are left to write contiguous digits
/// (no vertical by horizontal crossings; no horizontal overlaps:
/// each digit part of a single number)
/// - ascii input
#[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay03> {
        let mut numbers = NumberRegister::new();
        let mut adjacencies = SpecialAdjacenciesRegister::new();

        // register numbers & special chars
        input.lines().enumerate().for_each(|(row, raw_line)| {
                numbers.register_numbers(row as i64, raw_line);
                adjacencies.register_special_adjacencies(row as i64, raw_line);
        });

        info!("numbers: {:?}", numbers);
        info!("adjacencies: {:?}", adjacencies);
        // check what numbers are adjacent
        let mut sum = 0;
        for number in numbers {
                info!("number: {:?}", number.val());
                for location in number.locations() {
                        if adjacencies.contains(location) {
                                sum += number.val();
                                break;
                        }
                }
        }
        Ok(sum)
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        467..114..
                        ...*......
                        ..35..633.
                        ......#...
                        617*......
                        .....+.58.
                        ..592.....
                        ......755.
                        ...$.*....
                        .664.598..
                "};
                let expected = 4361;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        /// This test's expected value is to be populated after
        /// verification of solution.
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input1.txt");
                let expected = 535351;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
