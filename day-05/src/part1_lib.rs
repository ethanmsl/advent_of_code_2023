//! Library code for Part 1 of Day 05 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay05;
use core::cell::OnceCell;
use derive_more::{Constructor, IntoIterator};
use itertools::Itertools;
use miette::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;
use tracing::{debug, info, trace};

// Capture Patterns for Regex generation
static RE_SEEDS: Lazy<Regex> = Lazy::new(|| Regex::new(r"seeds: (?<seednumbers>.*)$").unwrap());
static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static RE_A_TO_B: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?<input>\w+)-to-(?<output>\w+) map:").unwrap());
static RE_VAL_MAP: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(?<outstart>\d+) (?<instart>\d+) (?<length>\d+)").unwrap());

/// Return lowest "seed" to "location" mapping's location value.
/// Parse seeds, maps kinds, and value map ranges.
///
/// ## NOTES:
/// - Values are approaching u32 MAX at least.
///   - u32::MAX ==              4_294_967_295
///   - i64::MAX == 18_446_744_073_709_551_615
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
        // splitting input along blank lines (blank lines discarded)
        let mut it_chunk = input.split("\n\n");
        let first_line = it_chunk.next().expect("empty input");

        let seeds: Vec<DynThings> = read_seeds(first_line).ok_or(AocErrorDay05::SeedsParse(
                "Failed to parse seeds".to_string(),
        ))?;
        debug!("seeds: {:?}", seeds);

        let maps: Vec<Map> = it_chunk.map(|chunk| Map::from_str(chunk)).collect();

        seeds.iter()
                .map(|seed| {
                        maps.iter().fold(seed.val, |mut acc, map| {
                                let temp = map.val_only_passthrough(acc);
                                info!(?acc, ?temp);
                                temp
                        })
                })
                .min()
                .ok_or(AocErrorDay05::MinFailure(
                        "Failed to find minimum after seed mapping".to_string(),
                ))
}

/// Read a single line string and extract seed values.
fn read_seeds(line: &str) -> Option<Vec<DynThings>> {
        const SEED: &str = "seed";
        let Some(seeds) = RE_SEEDS.captures(line) else {
                return None;
        };

        Some(RE_NUM
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().expect("parse failure"))
                .map(|val| DynThings::new(SEED.to_string(), val))
                .collect())
}

/// input & output values and 'range-bumps' that may modify an object
/// PERF: replace Strings with &str referencing static strings.
#[derive(Debug, PartialEq, Eq, Constructor)]
struct Map {
        inp: String,
        out: String,
        rmaps: Vec<RangeBump>,
}

impl Map {
        /// Passes a value in and passes it back, incremented if it matches any of the ranges.
        /// With**out** validating input kind or relaying output kind.
        fn val_only_passthrough(&self, val: i64) -> i64 {
                self.rmaps
                        .iter()
                        .filter_map(|rmap| rmap.try_bump(val))
                        .next()
                        .unwrap_or(val)
        }

        // Populate a map from a contiguous chunk of map string data.
        fn from_str(chunk: &str) -> Self {
                let mut lines = chunk.lines();
                let mut rmaps: Vec<RangeBump> = Vec::new();
                let first_line = lines.next().expect("empty chunk");
                trace!("first_line: {:?}", first_line);

                let caps = RE_A_TO_B.captures(first_line).expect("invalid map header");
                let inp = caps
                        .name("input")
                        .expect("invalid 'input' map header")
                        .as_str()
                        .to_string();
                let out = caps
                        .name("output")
                        .expect("invalid 'output' map header")
                        .as_str()
                        .to_string();
                lines.for_each(|line| {
                        let caps = RE_VAL_MAP.captures(line).expect("invalid map line");
                        let in_start = caps
                                .name("instart")
                                .expect("in_start")
                                .as_str()
                                .parse::<i64>()
                                .expect("instart parse failure");
                        let out_start = caps
                                .name("outstart")
                                .expect("outstart")
                                .as_str()
                                .parse::<i64>()
                                .expect("outstart parse failure");
                        let length = caps
                                .name("length")
                                .expect("length")
                                .as_str()
                                .parse::<i64>()
                                .expect("length parse failure");

                        rmaps.push(RangeBump::new(
                                (out_start - in_start),
                                (in_start..(in_start + length)),
                        ));
                });

                Self::new(inp, out, rmaps)
        }
}
// let caps = re.captures(hay).unwrap();
// assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
// assert_eq!(caps.name("title").unwrap().as_str(), "Citizen Kane");
// assert_eq!(caps.name("year").unwrap().as_str(), "1941");
// // You can also access the groups by name using the Index notation.
// // Note that this will panic on an invalid group name. In this case,
// // these accesses are always correct because the overall regex will
// // only match when these capture groups match.
// assert_eq!(&caps[0], "'Citizen Kane' (1941)");
// assert_eq!(&caps["title"], "Citizen Kane");
// assert_eq!(&caps["year"], "1941");

/// A range of values and the amount they are bumped by.
#[derive(Debug, PartialEq, Eq, Constructor)]
struct RangeBump {
        offset: i64,
        range: Range<i64>,
}

impl RangeBump {
        /// Returns true if the given value is in the range.
        /// If so, the offset is applied to the value.
        fn try_bump(&self, val: i64) -> Option<i64> {
                debug!(?val, ?self);
                if self.range.contains(&val) {
                        Some(val + self.offset)
                } else {
                        None
                }
        }
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
        val: i64,
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init(); // RUST_LOG=...

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

        /// This test's expected value is to be populated after
        /// verification of solution.
        /// (useful for future refactors and perfs)
        /// NOTE: `#[ignore]` is set for this test by default.
        // #[ignore]
        #[test]
        fn test_process_problem_input() -> Result<()> {
                let file_input = include_str!("../input1.txt");
                let expected = 462648396;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
