use std::collections::BTreeMap;

use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore, hash::md5};

struct Puzzle {
    seed: String,
}

impl Puzzle {
    fn brute_force_search_in_range(
        &self,
        start: usize,
        chunk_size: usize,
    ) -> Vec<(usize, [u8; 16])> {
        let bitmask = vec![0xff, 0xff, 0xf0];
        (start..start + chunk_size)
            .into_par_iter()
            .filter_map(|num| {
                let message = self.seed.clone() + &num.to_string();
                let digest = md5::digest(message.as_bytes());
                digest
                    .iter()
                    .zip(&bitmask)
                    .all(|(d, c)| d & c == 0)
                    .then_some((num, digest))
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let seed = input.trim().to_string();
        Ok(Self { seed })
    }

    fn part1(&self) -> String {
        let mut results = Vec::with_capacity(16);
        let chunk_size = 1_000_000;
        for start_idx in (0..).step_by(chunk_size) {
            if results.len() >= 8 {
                break;
            }
            let mut found = self
                .brute_force_search_in_range(start_idx, chunk_size)
                .into_iter()
                .map(|(idx, bytes)| (idx, bytes[2] & 0xf))
                .collect();
            results.append(&mut found);
        }
        results.sort_unstable_by_key(|&(idx, _)| idx);
        results
            .into_iter()
            .take(8)
            .fold(String::new(), |s, (_, byte)| s + &format!("{byte:1x}"))
    }

    fn part2(&self) -> String {
        let mut results = BTreeMap::new();
        let chunk_size = 1_000_000;
        for start_idx in (0..).step_by(chunk_size) {
            if results.len() >= 8 {
                break;
            }
            let mut found = self
                .brute_force_search_in_range(start_idx, chunk_size)
                .into_iter()
                .filter_map(|(idx, bytes)| {
                    (bytes[2] & 0xf < 8).then_some((idx, (bytes[2] & 0xf, bytes[3] >> 4 & 0xf)))
                })
                .collect::<Vec<_>>();
            found.sort_unstable_by_key(|&(idx, _)| idx);
            for (_, (idx, byte)) in found {
                results.entry(idx).or_insert(byte);
            }
        }
        results
            .into_iter()
            .take(8)
            .fold(String::new(), |s, (_, byte)| s + &format!("{byte:1x}"))
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
