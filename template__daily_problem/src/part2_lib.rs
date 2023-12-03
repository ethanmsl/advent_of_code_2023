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
        fn test_process() -> Result<()> {
                todo!("haven't built test yet");
                let input = "";
                assert_eq!(process(input)?, "");
                Ok(())
        }
}
