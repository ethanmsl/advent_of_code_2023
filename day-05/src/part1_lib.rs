//! Library code for Part 1 of Day 05 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay05;
use core::cell::OnceCell;
use derive_more::{Constructor, IntoIterator};
use miette::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

// Capture Patterns for Regex generation
static RE_SEEDS: Lazy<Regex> = Lazy::new(|| Regex::new(r"seeds: (.*)$").unwrap());
static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static RE_A_TO_B: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)-to-(\w+):").unwrap());
static RE_VAL_MAP: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+) (\d+) (\d+)").unwrap());

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

/// ## Design Thoughts:
/// A natural desire for this implementation would be an 'enum' of the map kinds.
/// However, those are dynamically determined.  A lack of subtyping would also make a struct
/// like "Seeds" difficult and reliant on user execution of validators, I believe.
///
/// On continued thought, "Seeds doesn't have much use here other than gating propogation, which
/// isn't really a natural behavior of the maps object, nor one I would want to implement."
/// (I suppose it is natural in the context of a vector based implementation that discards actual
/// to from tags)
// #[tracing::instrument]
pub fn process(input: &str) -> Result<i64, AocErrorDay05> {
        info!("Hiii. from  day-05 Part1! :)");
        let mut it_inp = input.lines();
        let first_line = it_inp.next().expect("empty input");

        let seeds: Vec<DynThings> = read_seeds(first_line).ok_or(AocErrorDay05::SeedsParse(
                "Failed to parse seeds".to_string(),
        ))?;
        info!("seeds: {:?}", seeds);
        // let maps: Maps = it_inp.collect();
        //
        // seeds.iter().map(|seed| maps.propogate_complete(seed) ).min()
        // let seeds_w = maps.propogate_complete(seeds);
        todo!("day 05 - Part 1");
}

fn read_seeds(line: &str) -> Option<Vec<DynThings>> {
        const SEED: &str = "seed";
        let Some(seeds) = RE_SEEDS.captures(line) else {
                return None;
        };

        Some(RE_NUM
                .find_iter(line)
                .map(|m| m.as_str().parse::<u64>().expect("parse failure"))
                .map(|val| DynThings::new(SEED.to_string(), val))
                .collect())
}

/// Object with dynamic 'kind' indicated by a string and a positive value.
///
/// PERF: there ought to be a way to write a single static string that they can all reference.
/// Or perhaps a collection of them, they can reference from...
/// For now, the number of items is so small that I'll just use a whole string, but this is
/// expensive.
#[derive(Debug, PartialEq, Eq, Constructor)]
struct DynThings {
        kind: String,
        val: u64,
}

// parser
// // seeds:
//
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
