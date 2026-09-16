use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Puzzle {
    strings: Vec<String>,

    // const but need to be initialized in the constructor
    vowels: BTreeSet<char>,
    forbidden: BTreeSet<(char, char)>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let strings = parser::parse_lines(input.trim(), |line| Ok(line.to_string()))?;
        let vowels = BTreeSet::from_iter(['a', 'e', 'i', 'o', 'u']);
        let forbidden = BTreeSet::from_iter([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
        Ok(Self {
            strings,
            vowels,
            forbidden,
        })
    }

    fn part1(&self) -> String {
        let is_nice = |s: &&String| {
            let s = s.chars().collect::<Vec<_>>();

            s.iter().filter(|c| self.vowels.contains(c)).count() >= 3
                && s.array_windows::<2>().any(|&[left, right]| left == right)
                && s.array_windows::<2>()
                    .all(|&pair| !self.forbidden.contains(&pair.into()))
        };
        self.strings.par_iter().filter(is_nice).count().to_string()
    }

    fn part2(&self) -> String {
        let is_nice = |s: &&String| {
            let s = s.chars().collect::<Vec<_>>();

            let has_repeated_pair = {
                let mut pair_count = BTreeMap::new();
                let mut has_repeated_pair = false;
                for (idx, pair) in s.array_windows::<2>().enumerate() {
                    let last_pos = pair_count.entry(pair).or_insert(idx);
                    if idx - *last_pos > 1 {
                        has_repeated_pair = true;
                        break;
                    }
                }
                has_repeated_pair
            };
            has_repeated_pair
                && s.array_windows::<3>()
                    .any(|&[left, _, right]| left == right)
        };
        self.strings.par_iter().filter(is_nice).count().to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
