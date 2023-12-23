//! Part 2 of Day 08 for Advent of Code 2023.
//! Runs `part2_lib_numtheory.rs` library code against `input2.txt`

use anyhow::{Context, Result};
use day_08::part2_lib_numtheory::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> Result<()> {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        #[cfg(not(feature = "dhat-heap"))]
        tracing_subscriber::fmt::init();

        let file = include_str!("../../input2.txt");
        let result = process(file).context("process part 2")?;
        println!("\n************\nSolution is:\n{}", result);
        Ok(())
}
