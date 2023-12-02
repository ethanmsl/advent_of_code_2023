//! Custom Error type for {{ project-name | title_case }} of Advent of Code 2023.

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError{{project-name | upper_camel_case}} {
        #[error(transparent)]
        #[diagnostic(code(aoc::io_error))]
        IoError(#[from] std::io::Error),
}
