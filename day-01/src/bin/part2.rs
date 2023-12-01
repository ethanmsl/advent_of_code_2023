//! Library code for Part 2 of Advent of Code 2023 for Day-01
//! `bin > part2.rs` will run this code along with conent of `input2.txt`

use day_01::part2::process;
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

        let file = include_str!("../../input2.txt");
        let result = process(file).context("process part 2")?;
        println!("\n************\nSolution is:\n{}", result);
        Ok(())
}
