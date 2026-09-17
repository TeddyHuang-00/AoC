//! Utilities for Advent of Code challenges

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

#[macro_export]
macro_rules! chore {
    ($day:expr) => {
        const YEAR_N_DAY: &str = $day;
        const PUZZLE_INPUT: &str = include_str!("../puzzle.in");
        const EXAMPLE_INPUT: &str = include_str!("../example.in");
        const EXAMPLE_OUTPUT: &str = include_str!("../example.out");

        impl util::InputData for Puzzle {
            const INPUT: &str = PUZZLE_INPUT;
            const EXAMPLE: &str = EXAMPLE_INPUT;
        }

        impl Puzzle {
            fn new() -> Result<Self> {
                use util::InputData;
                Self::parse::<false>(Puzzle::INPUT)
            }

            fn example() -> Result<Self> {
                use util::InputData;
                Self::parse::<true>(Puzzle::EXAMPLE)
            }
        }

        fn main() -> anyhow::Result<()> {
            use util::InputData;

            let puzzle = Puzzle::new()?;
            let year = &YEAR_N_DAY[1..5];
            let day = &YEAR_N_DAY[6..];
            println!("Year {year} Day {day} Part 1: {}", puzzle.part1());
            println!("Year {year} Day {day} Part 2: {}", puzzle.part2());

            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use std::time::Duration;

            use util::{Benchmark, Serializable};

            use super::*;

            fn split_answers(output: &str) -> (&str, &str) {
                let mut lines = output.lines();
                let p1 = lines.next().unwrap_or("").trim();
                let p2 = lines.next().unwrap_or("").trim();
                (p1, p2)
            }

            #[test]
            fn test_part1() -> anyhow::Result<()> {
                let puzzle = Puzzle::example()?;
                assert_eq!(puzzle.part1(), split_answers(EXAMPLE_OUTPUT).0);
                Ok(())
            }

            #[test]
            fn test_part2() -> anyhow::Result<()> {
                let puzzle = Puzzle::example()?;
                assert_eq!(puzzle.part2(), split_answers(EXAMPLE_OUTPUT).1);
                Ok(())
            }

            #[test]
            fn benchmark() -> Result<()> {
                Puzzle::bench_all(Duration::from_secs(1)).to_csv()
            }
        }
    };
}
