//! Custom Error type for Day 06 of Advent of Code 2023.

// use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocErrorDay06 {
        #[error(transparent)]
        IoError(#[from] std::io::Error),

        #[error("Error parsing seeds: {0}")]
        ParseError(String),
        // #[error("Error parsing seeds: {0}")]
        // #[diagnostic(code(aoc::min_unwrap_error))]
        // MinFailure(String),
}
