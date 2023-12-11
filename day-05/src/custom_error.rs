//! Custom Error type for Day 05 of Advent of Code 2023.

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocErrorDay05 {
        #[error(transparent)]
        #[diagnostic(code(aoc::io_error))]
        IoError(#[from] std::io::Error),

        #[error("Error parsing seeds: {0}")]
        #[diagnostic(code(aoc::seeds_parse_error))]
        SeedsParse(String),

        #[error("Error parsing seeds: {0}")]
        #[diagnostic(code(aoc::min_unwrap_error))]
        MinFailure(String),
}
