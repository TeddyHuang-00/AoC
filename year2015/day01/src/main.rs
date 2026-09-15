use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Puzzle {
    seq: Vec<i8>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let seq = parser::parse_chars(input.trim(), |char| match char {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => anyhow::bail!("Invalid input"),
        })?;
        Ok(Self { seq })
    }

    fn part1(&self) -> String {
        self.seq
            .par_iter()
            .copied()
            .map(i32::from)
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.seq
            .iter()
            .copied()
            .scan(0, |acc, x| {
                *acc += i32::from(x);
                Some(*acc)
            })
            .enumerate()
            .find(|&(_, level)| level == -1)
            .map_or_else(
                || "No basement found".to_string(),
                |(index, _)| (index + 1).to_string(),
            )
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
