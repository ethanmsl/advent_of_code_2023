//! Library code for Part 2 of Day 02 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use crate::custom_error::AocErrorDay02;
use miette::Result;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocErrorDay02> {
        todo!("day 02 - part 2");
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
