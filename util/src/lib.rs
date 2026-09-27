//! Utilities for Advent of Code challenges

// re-exporting all public macros
#[macro_use]
mod macros;

pub mod hash;
pub mod parser;
pub mod timer;
pub mod vector;
pub mod writer;

use std::time::Duration;

use anyhow::Result;

use crate::timer::{BenchmarkResult, measure_many};
pub use crate::writer::Serializable;

/// A trait that is used to associate input data with a specific solution
/// struct.
pub trait InputData {
    const INPUT: &'static str;
    const EXAMPLE: &'static str;
}

/// A trait that defines the structure for an Advent of Code solution.
pub trait Solution {
    /// Parse the input data for the day's challenge.
    fn parse<const E: bool>(input: &'static str) -> Result<Self>
    where
        Self: Sized;

    /// Solve part 1 of the day's challenge.
    ///
    /// Should handle errors internally and return the result as a String.
    fn part1(&self) -> String;

    /// Solve part 2 of the day's challenge.
    ///
    /// Should handle errors internally and return the result as a String.
    fn part2(&self) -> String;
}

pub trait Benchmark {
    fn bench_parse(time_limit: Duration) -> BenchmarkResult;
    fn bench_part1(time_limit: Duration) -> BenchmarkResult;
    fn bench_part2(time_limit: Duration) -> BenchmarkResult;
    #[must_use]
    fn bench_all(time_limit: Duration) -> [BenchmarkResult; 3] {
        [
            Self::bench_parse(time_limit),
            Self::bench_part1(time_limit),
            Self::bench_part2(time_limit),
        ]
    }
}

impl<T: Solution + InputData> Benchmark for T {
    fn bench_parse(time_limit: Duration) -> BenchmarkResult {
        measure_many("Parse", time_limit, || T::parse::<false>(T::INPUT))
    }

    fn bench_part1(time_limit: Duration) -> BenchmarkResult {
        let puzzle = T::parse::<false>(T::INPUT)
            .unwrap_or_else(|err| panic!("Failed to parse input: {err}"));
        measure_many("Part 1", time_limit, move || puzzle.part1())
    }

    fn bench_part2(time_limit: Duration) -> BenchmarkResult {
        let puzzle = T::parse::<false>(T::INPUT)
            .unwrap_or_else(|err| panic!("Failed to parse input: {err}"));
        measure_many("Part 2", time_limit, move || puzzle.part2())
    }
}
