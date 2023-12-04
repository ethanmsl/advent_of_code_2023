//! Library code for Part 1 of Day 03 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocErrorDay03;
use miette::Result;
use regex::Regex;
use test_case::test_case;
use tracing::info;

/// Return sum of values adjacent to special chars
///
/// ## Proposed Flow
/// - get number locations --> map each to number
/// - get special locations --> calculate adjacency locations
/// - use adjacencies to look up numbers
///
/// ## Assumptions
/// - Numbers are left to write contiguous digits
/// (no vertical by horizontal crossings; no horizontal overlaps:
/// each digit part of a single number)
/// - ascii input
#[tracing::instrument]
pub fn process(_input: &str) -> Result<u64, AocErrorDay03> {
        todo!("day 03 - part 1");
}

// #[test_case(".....+.58." => (0,10); ":shrug:")]
#[test_case("0123XXX789" => (4,7); ":shrug:2")]
fn find_number(hay: &str) -> (usize, usize) {
        tracing_subscriber::fmt::init();

        let pat = r"X+";
        let re = Regex::new(pat).unwrap();
        let m = re.find(hay).unwrap();
        info!("{}", &hay[m.start()..m.end()]);
        info!("{}", m.as_str());
        (m.start(), m.end())
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
                let expected = 114;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        /// This test's expected value is to be populated after
        /// verification of solution.  
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        #[ignore]
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input1.txt");
                let expected = todo!();
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
