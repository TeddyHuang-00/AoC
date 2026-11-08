use std::collections::BTreeMap;

use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    ids: Vec<&'static str>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let ids = input.trim().lines().collect();
        Ok(Self { ids })
    }

    fn part1(&self) -> String {
        let (any2, any3) = self.ids.iter().fold((0, 0), |(mut a2, mut a3), &s| {
            let count = s.chars().fold(BTreeMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            });
            if count.values().any(|&cnt| cnt == 2) {
                a2 += 1;
            }
            if count.values().any(|&cnt| cnt == 3) {
                a3 += 1;
            }
            (a2, a3)
        });
        (any2 * any3).to_string()
    }

    fn part2(&self) -> String {
        for (i, &seq1) in self.ids.iter().enumerate() {
            for &seq2 in &self.ids[(i + 1)..] {
                let common = seq1
                    .chars()
                    .zip(seq2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .count();
                if common == seq1.len() - 1 {
                    return seq1
                        .chars()
                        .zip(seq2.chars())
                        .filter_map(|(c1, c2)| (c1 == c2).then_some(c1))
                        .collect();
                }
            }
        }
        panic!("No similar ID found")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
