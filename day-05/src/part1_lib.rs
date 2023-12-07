//! Library code for Part 1 of Day 05 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay05;
use miette::Result;
use tracing::info;

// Capture Patterns for Regex generation
const SEEDS: &str = r"seeds: (.*)$";
const NUM: &str = r"(\d)";
const A_TO_B: &str = r"(\w+)-to-(\w):";
const VAL_MAP: &str = r"^(\d+) (\d+) (\d+)";

/// Return lowest "seed" to "location" mapping's location value.
/// Parse seeds, maps kinds, and value map ranges.
///
/// ## NOTES:
/// - Values are approaching u32 MAX at least.
///   - u32::MAX ==              4_294_967_295
///   - u64::MAX == 18_446_744_073_709_551_615
///   - sample_input_val ==      2_906_633_798
/// - Part1 mappings appear to be ordered.
///   - e.g. a to b, b to c, c to d
///   - We could use a vector and skip mapping kind parsing or lookup.
///   - alt: we could HashMap for generality
///     - alt, alt: we could BTree to 'take the difference'
///       - ^ probably best from general edu-spirit of the games
///     - there's also an 'ordered-hashmap crate out there'
///       - **EDIT**: renamed to `IndexMap`
///
/// ## Questions:
/// - Use `Logos` for primitive parsing?
///   - (good chance to work with non-shallow enums)
///
/// ## General Path:
/// - Parse
///   - first line to seeds
///   - chunk inputs based on blank lines
///   - parse a-to-b (not needed for part1, but still)
///   - pars raw values into input range + offset
/// - Just iterate through remaining
///   - see if value is in a range
///   - apply offset if so
///   - continue
/// - Return lowest when done

// #[tracing::instrument]
pub fn process(_input: &str) -> Result<i64, AocErrorDay05> {
        info!("Hiii. from  day-05 Part1! :)");
        todo!("day 05 - Part 1");
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        seeds: 79 14 55 13

                        seed-to-soil map:
                        50 98 2
                        52 50 48

                        soil-to-fertilizer map:
                        0 15 37
                        37 52 2
                        39 0 15

                        fertilizer-to-water map:
                        49 53 8
                        0 11 42
                        42 0 7
                        57 7 4

                        water-to-light map:
                        88 18 7
                        18 25 70

                        light-to-temperature map:
                        45 77 23
                        81 45 19
                        68 64 13

                        temperature-to-humidity map:
                        0 69 1
                        1 0 69

                        humidity-to-location map:
                        60 56 37
                        56 93 4
                "};
                let expected = 35;
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
        //         let file_input = include_str!("../input1.txt");
        //         let expected = todo!();
        //         assert_eq!(process(file_input)?, expected);
        //         Ok(())
        // }
}
