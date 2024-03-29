//! Library code for Part 2 of Day 02 of Advent of Code 2023.
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use anyhow::Result;

use crate::custom_error::AocErrorDay02;

/// Example helpfully provided by the `once_cell` crate's documentation.
///
/// Simply assigns a string literal pattern to a `Regex` generator
/// wrapped in a lazy `once_cell` wrapper.
/// Besides avoiding perf issues with accidental-recompilation
/// I just find using a `static` for these patterns clear and readable for this code.
///
/// # PERF: `regex-automata` crate allows creating a DFA, writing it, and rolling it into binary.
///  That woudl avoid any initialization cost when running binary, and allow speed optimization
///  for what are very simple regexes.
// #[macro_export]
macro_rules! regex_lazyonce {
        ($re:expr $(,)?) => {{
                static RE: once_cell::sync::OnceCell<regex::Regex> =
                        once_cell::sync::OnceCell::new();
                RE.get_or_init(|| regex::Regex::new($re).unwrap())
        }};
}

// // for testing for pathological inputs
// const DUMB_PAT: &str = r"(\d+) (\w+), (\d+) (\w+), (\d+) (\w+), (\d+) (\w+);";
//                 regex_lazyonce!(DUMB_PAT).captures_iter(line).for_each(|c| {
//                         let (needle, _): (&str, [&str; 6]) = c.extract();
//                         info!("{:?}", needle);
//                 });

// const ID_PAT: &str = r"Game (\d+):";
const RED_PAT: &str = r"(\d+) red";
const GREEN_PAT: &str = r"(\d+) green";
const BLUE_PAT: &str = r"(\d+) blue";

/// set of cubes
#[derive(Debug, PartialEq)]
struct Cubes {
        red:   u64,
        green: u64,
        blue:  u64,
}

impl Cubes {
        /// whether a is a ~subset (inclusive) of b
        fn pow_max(&self) -> u64 {
                self.red * self.green * self.blue
        }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<u64, AocErrorDay02> {
        let pows_sum = input.lines()
                            .map(|line| {
                                    let line_cubes_maxes = extract_data_noid(line);
                                    line_cubes_maxes.pow_max()
                            })
                            .sum();
        Ok(pows_sum)
}

/// REFACTOR:
/// yuuuch at repeated code.
/// Not sure of  a better way to do this if usign regex_macro
/// ... I should just have defined the regex as static with lazy
/// and then iterated over them.
fn extract_data_noid(hay: &str) -> Cubes {
        let r_sum = regex_lazyonce!(RED_PAT).captures_iter(hay)
                                            .map(|c| {
                                                    let (_, [val]) = c.extract();
                                                    val.parse::<u64>()
                                                       .expect("red parse failure")
                                            })
                                            .max()
                                            .unwrap_or(0);
        let g_sum = regex_lazyonce!(GREEN_PAT).captures_iter(hay)
                                              .map(|c| {
                                                      let (_, [val]) = c.extract();
                                                      val.parse::<u64>()
                                                         .expect("green parse failure")
                                              })
                                              .max()
                                              .unwrap_or(0);
        let b_sum = regex_lazyonce!(BLUE_PAT).captures_iter(hay)
                                             .map(|c| {
                                                     let (_, [val]) = c.extract();
                                                     val.parse::<u64>()
                                                        .expect("blue parse failure")
                                             })
                                             .max()
                                             .unwrap_or(0);

        Cubes { red:   r_sum,
                green: g_sum,
                blue:  b_sum, }
}
#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_process_example() -> Result<()> {
                tracing_subscriber::fmt::init();

                let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
                let expected = 2286;
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
                let file_input = include_str!("../input2.txt");
                let expected = 65371;
                assert_eq!(process(file_input)?, expected);
                Ok(())
        }
}
