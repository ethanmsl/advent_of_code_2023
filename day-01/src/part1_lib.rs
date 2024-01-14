//! Library code for Part 1 of Day 01 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use miette::Result;

use crate::custom_error::AocError;

/// Take first and last digit char (may be the same!)
/// from each line to form a number.
/// Return that sum.
#[tracing::instrument]
pub fn process(input: &str) -> Result<u32, AocError> {
        let mut nums = Vec::<u32>::new();

        // lines
        for ln in input.lines() {
                // ascii digits
                let chars: Vec<char> = ln.chars()
                                         .filter(|c| c.is_ascii_digit())
                                         .collect();

                // extracting, formatting, parsing, pushing
                if let (Some(&first), Some(&last)) = (chars.first(), chars.last()) {
                        let str_num = format!("{}{}", first, last);
                        match str_num.parse::<u32>() {
                                Ok(num) => nums.push(num),
                                Err(_) => {
                                        panic!("Could not parse: {} \nfrom line: {}", str_num, ln)
                                },
                        }
                }
        }

        Ok(nums.into_iter().sum())
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
                let expected = 142;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        /// This test's expected value is to be populated after
        /// verification of solution.  
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input1.txt");
                let expected = 55621;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
