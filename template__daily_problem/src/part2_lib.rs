//! Library code for Part 2 of {{ project-name | title_case }} of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use crate::custom_error::AocError{{project-name | upper_camel_case}};
use miette::Result;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError{{project-name | upper_camel_case}}> {
        todo!("{{project-name | title_case | downcase}} - part 2");
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = "";
                let expected = todo!();
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
                let file_input = include_str!("../input2.txt");
                let expected = todo!();
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
