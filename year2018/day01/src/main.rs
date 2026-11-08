use std::{collections::HashSet, hash::BuildHasherDefault};

use anyhow::Result;
use nom::{
    character::complete::{i32, line_ending},
    error,
    multi::separated_list1,
};
use util::{Solution, chore, hasher::FastHasher, parser};

struct Puzzle {
    freq_changes: Vec<i32>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let freq_changes = parser::parse_input_str(
            input,
            separated_list1(line_ending::<&str, error::Error<&str>>, i32),
        )?;
        Ok(Self { freq_changes })
    }

    fn part1(&self) -> String {
        self.freq_changes.iter().copied().sum::<i32>().to_string()
    }

    fn part2(&self) -> String {
        let mut seen = HashSet::with_hasher(BuildHasherDefault::<FastHasher>::new());
        let mut last = 0;
        seen.insert(0);
        for fc in self.freq_changes.iter().cycle() {
            last += fc;
            if !seen.insert(last) {
                break;
            }
        }
        last.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
