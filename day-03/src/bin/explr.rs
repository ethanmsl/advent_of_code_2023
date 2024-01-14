//! Library code for Part 1 of Day 03 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use regex::Regex;
use tracing::info;

/// Return sum of values adjacent to special chars
///
/// ## Proposed Flow
/// - get number locations --> map each to number
/// - get special locations --> calculate adjacency locations
/// - use adjacencies to look up numbers
///
/// ## Assumptions
/// - Numbers are left to write contiguous digits
/// (no vertical by horizontal crossings; no horizontal overlaps:
/// each digit part of a single number)
/// - ascii input

/// give verious match info from a regex search
/// for the first pattern matching repeated digits
fn check_regex_match_behavior(hay: &str) -> (usize, usize) {
        let pat = r"\d+";
        let re = Regex::new(pat).unwrap();
        let m = re.find(hay).unwrap();
        info!("input[{}..{}]: {}",
              m.start(),
              m.end(),
              &hay[m.start()..m.end()]);
        info!("input.as_str(): {}", m.as_str());
        info!("input.len(): {}", m.len());
        (m.start(), m.end())
}

fn main() {
        tracing_subscriber::fmt::init();

        let hay = "oiy717cbd";
        dbg!(check_regex_match_behavior(hay));
}
