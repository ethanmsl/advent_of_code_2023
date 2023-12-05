//! Library code for Part 2 of Day 03 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use crate::custom_error::AocErrorDay03;
use crate::data_types_part2::{NumberRegister, StarAndAdjacenciesRegister};
use miette::Result;
use std::collections::HashMap;
use tracing::info;

/// Basically we need to do the same thing, but now we need
/// run across a specific special char's adjacencies
/// Find numbers adjacent. and if precisely two get it's product
/// (adn then sum or something)
#[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay03> {
        let mut numbers = NumberRegister::new();
        let mut star_adjacencies = StarAndAdjacenciesRegister::new();

        // register numbers & special chars
        input.lines().enumerate().for_each(|(row, raw_line)| {
                numbers.register_numbers(row as i64, raw_line);
                star_adjacencies.register_special_adjacencies(row as i64, raw_line);
        });

        // check what numbers are adjacent to each star
        let mut sum = 0;
        for (id, locations) in star_adjacencies.hmap {
                let mut adjacent_nums = HashMap::new();
                'inner: for location in locations {
                        match numbers.hmap.get(&location) {
                                Some(numinf) => adjacent_nums.insert(numinf.id(), numinf.val()),
                                None => continue 'inner,
                        };
                }
                if adjacent_nums.len() == 2 {
                        let product = adjacent_nums.values().product::<u64>();
                        info!(?id, ?product);
                        sum += product;
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
                let expected = 467835;
                assert_eq!(process(input)?, expected);
                Ok(())
        }

        /// This test's expected value is to be populated after
        /// verification of solution.
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input2.txt");
                let expected = 87287096;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
