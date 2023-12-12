//! Library code for Part 1 of Day 06 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`
#![allow(warnings)]

use crate::custom_error::AocErrorDay06;
use anyhow::Result;
use derive_more::Constructor;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::ops::RangeInclusive;
use tracing::{debug, info, trace};

static RE_TIME: Lazy<Regex> = Lazy::new(|| Regex::new(r"Time: (?<time>.*)$").unwrap());
static RE_DIST: Lazy<Regex> = Lazy::new(|| Regex::new(r"Distance: (?<dist>.*)$").unwrap());
static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

// #[tracing::instrument]
/// Return all integer pairs less oblong than those of 'record' labelled by "Distance".
///
/// ## Example
/// Time: 7, Distance: 9
/// Find  x * (7 - x) == 9
/// get int_ceil(x)
/// Solution = Time -2x (where x is the edges we wouldn't compete in)
///
/// ## Approaches:
/// - We coudl work with floating points and do calculus.
/// - We can brute force explore integer pairs until we beat it.
///   - Biggest input number is 94, and max search is less than half of that. (if we assume all are
///   winnable)
/// - We can do some algebra and just solve a quadratic equation. (e.g. -9 + 7x x^2 == 0)
///  - ( -dist + time *x - x^2 )
///
/// Given that we're a bit behidn brute force is probably simplest given the nature of the problem.
/// Or we could just look up the quatdratic solution formula -- lol :
///   - `x = (-b +/- sqrt(b^2 - 4ac)) / 2a`
///
pub fn process(input: &str) -> Result<usize> {
        let lb = lower_bound_solution(30, 200);
        info!(lb);
        info!("Hiii. from  day-06 Part1! :)");
        let stats: Vec<GameStats> = input_to_games(input)?;
        info!("Stats: {:#?}", stats);

        Ok(stats.iter()
                .map(|s| {
                        let lb = lower_bound_solution(s.max_time, s.record_dist);
                        lb_to_count(s.max_time, lb)
                })
                .inspect(|c| info!("Count: {}", c))
                .product())
}

/// Hard coding input.
fn input_to_games(inp: &str) -> Result<Vec<GameStats>> {
        let mut lines = inp.lines();
        let line = lines.next().expect("missing line 1");
        let Some(_) = RE_TIME.captures(line) else {
                return Err(AocErrorDay06::ParseError("Missing Time Line".to_string()))?;
        };
        let times: Vec<_> = RE_NUM
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().expect("parse failure"))
                .collect();

        let line = lines.next().expect("missing line 2");
        let Some(_) = RE_DIST.captures(line) else {
                return Err(AocErrorDay06::ParseError("Missing Dist. Line".to_string()))?;
        };
        let dists: Vec<_> = RE_NUM
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().expect("parse failure"))
                .collect();
        Ok(times.into_iter()
                .zip(dists.into_iter())
                .map(|(t, d)| GameStats::new(t as u64, d as u64))
                .collect())
}

/// Game's allowed time and best record distance.
#[derive(Debug, Constructor)]
struct GameStats {
        max_time: u64,
        record_dist: u64,
}

/// Takes lower_bound and max_time and returns the inner count of whole integers
fn lb_to_count(max_time: u64, lb: u64) -> usize {
        (lb..=(max_time - lb)).count()
}

///   Smallest solution to get the record boat score.
///   (Lower bound of winning numbers, with a mirroring upper bound)
///   
///   `x = (-b +/- sqrt(b^2 - 4ac)) / 2a`
///   `a x^2 + b x + c = 0`
fn lower_bound_solution(max_time: u64, record_dist: u64) -> u64 {
        let c_0 = -1.0 * record_dist as f64;
        let b_1 = max_time as f64;
        const a_2: f64 = -1.0;
        // problem requires two (pos) solutions to this to be winnable.
        // we can use the fact that that correspond to a discriminant > 0 to create a guard.
        if c_0 * a_2 >= b_1.powi(2) {
                if c_0 * a_2 == b_1.powi(2) {
                        panic!("Perfect, unbeatable score!")
                }
                panic!("Nonsense, impossible score!");
        }

        let discriminant = b_1.powi(2) - 4.0 * a_2 * c_0;
        let x_min = (-b_1 + discriminant.sqrt()) / (2.0 * a_2);
        // let x_max = (-b_1 - discriminant.sqrt()) / (2.0 * a_0);
        // (x_min.ceil() as i32..=(x_max.floor() as i32))
        x_min.ceil() as u64
}

#[cfg(test)]
mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = indoc! {"
                        Time:      7  15   30
                        Distance:  9  40  200
                "};
                let expected = 288;
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
