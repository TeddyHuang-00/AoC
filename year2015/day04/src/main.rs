use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore};

mod md5;

struct Puzzle {
    seed: String,
}

impl Puzzle {
    fn brute_force_search(&self, criterion_bitmask: &[u8]) -> usize {
        // We have to manually chunk the search space because
        // `find_first` does not work properly with infinite ranges.
        // So we will search in chunks of 1,000,000 numbers at a time instead.
        let chunk_size = 1_000_000;
        for start_idx in (0..).step_by(chunk_size) {
            let found = (start_idx..start_idx + chunk_size)
                .into_par_iter()
                .filter(|num| {
                    let message = self.seed.clone() + &num.to_string();
                    let digest = md5::digest(message.as_bytes());
                    digest
                        .iter()
                        .zip(criterion_bitmask)
                        .all(|(d, &c)| d & c == 0)
                })
                .min();
            if let Some(num) = found {
                return num;
            }
        }
        unreachable!("No valid number found");
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        Ok(Self {
            seed: input.trim().to_string(),
        })
    }

    fn part1(&self) -> String {
        // First five hex digits are zero
        self.brute_force_search(&[0xff, 0xff, 0xf0]).to_string()
    }

    fn part2(&self) -> String {
        // First six hex digits are zero
        self.brute_force_search(&[0xff, 0xff, 0xff]).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
