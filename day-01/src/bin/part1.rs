//! Part 1 of Day 01 of Advent of Code 2023.
//! `bin > part1.rs` will run this code along with conent of `input1.txt`

use day_01::part1_lib::process;
use miette::{Context, Result};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> Result<()> {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        #[cfg(not(feature = "dhat-heap"))]
        tracing_subscriber::fmt::init();

        let file = include_str!("../../input1.txt");
        let result = process(file).context("process part 1")?;
        println!("\n************\nSolution is:\n{}", result);
        Ok(())
}
