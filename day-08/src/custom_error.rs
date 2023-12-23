//! Custom Error type for Day 08 of Advent of Code 2023.

use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum AocErrorDay08 {
        IoError,
        SolutionsLengthMismatch,
}
