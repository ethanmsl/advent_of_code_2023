//! Library code for Part 1 of Advent of Code 2023 for Day-__.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use crate::custom_error::AocError;
use miette::Result;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AocError> {
        todo!("day __ - part 1");
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
                todo!("haven't built test yet");
                let input = "";
                assert_eq!(process(input)?, "");
                Ok(())
        }
}
