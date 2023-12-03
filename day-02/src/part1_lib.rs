//! Library code for Part 1 of Day 02 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocErrorDay02;
use miette::Result;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AocErrorDay02> {
        todo!("day 02 - part 1");
}

const BLUE_CAP_PATTERN: &str = r"(\d+) Blue";
const MAX_CUBES: Cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
};

/// set of cubes
struct Cubes {
        red: u32,
        green: u32,
        blue: u32,
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
                let expected = "8";
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
                let expected = "";
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
