//! Library code for Part 1 of Day 05 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use anyhow::Result;
use derive_more::Constructor;
use indoc::indoc;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, info, trace};

// Sample 'hay'
const TEXT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
"};

// Capture Patterns for Regex generation
const SEEDS_PAT: &str = r"seeds: (?<seednumbers>.*)$";
const NUM_PAT: &str = r"\d+";
const A_TO_B_PAT: &str = r"^(?<input>\d+) (?<output>\d+)$";
const VAL_MAP_PAT: &str = r"^(?<outstart>\d+) (?<instart>\d+) (?<length>\d+)";

// Sets up the Regexes
// I should probably use a macro for tidiness
static RE_SEEDS: Lazy<Regex> = Lazy::new(|| Regex::new(SEEDS_PAT).unwrap());
static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(NUM_PAT).unwrap());
static RE_A_TO_B: Lazy<Regex> = Lazy::new(|| Regex::new(A_TO_B_PAT).unwrap());
static RE_VAL_MAP: Lazy<Regex> = Lazy::new(|| Regex::new(VAL_MAP_PAT).unwrap());

fn main() -> Result<()> {
        tracing_subscriber::fmt::init();
        info!("starting");
        Ok(())
}

/// Read a single line string and extract seed values.
fn read_seeds(line: &str) -> Option<Vec<(String, u64)>> {
        const SEED: &str = "seed";
        let Some(_) = RE_SEEDS.captures(line)
        else {
                return None;
        };

        Some(RE_NUM.find_iter(line)
                   .map(|m| {
                           m.as_str()
                            .parse::<u64>()
                            .expect("parse failure")
                   })
                   .map(|val| (SEED.to_string(), val))
                   .collect())
}

// Populate a map from a contiguous chunk of map string data.
fn from_str(chunk: &str) -> () {
        let mut lines = chunk.lines();
        let first_line = lines.next()
                              .expect("empty chunk");
        trace!("first_line: {:?}", first_line);

        let caps = RE_A_TO_B.captures(first_line)
                            .expect("invalid map header");
        let inp = caps.name("input")
                      .expect("invalid 'input' map header")
                      .as_str()
                      .to_string();
        let out = caps.name("output")
                      .expect("invalid 'output' map header")
                      .as_str()
                      .to_string();

        lines.for_each(|line| {
                     let caps = RE_VAL_MAP.captures(line)
                                          .expect("invalid map line");
                     let in_start = caps.name("instart")
                                        .expect("in_start")
                                        .as_str()
                                        .parse::<i64>()
                                        .expect("instart parse failure");
                     let out_start = caps.name("outstart")
                                         .expect("outstart")
                                         .as_str()
                                         .parse::<i64>()
                                         .expect("outstart parse failure");
                     let length = caps.name("length")
                                      .expect("length")
                                      .as_str()
                                      .parse::<i64>()
                                      .expect("length parse failure");
             });
        ()
}
