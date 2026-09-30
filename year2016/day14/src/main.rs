use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use rayon::prelude::*;
use util::{
    Solution, chore,
    hash::{digest16_to_hex_byte, digest16_to_hex_char, md5},
};

struct Puzzle {
    salt: String,
}

macro_rules! all_eq {
    ($first:expr, $second:expr, $($rest:expr),+) => {
        $first == $second && all_eq!($second, $($rest),+)
    };
    ($first:expr, $second:expr) => {
        $first == $second
    };
}

impl Puzzle {
    fn find_keys(&self, extra_hashes: usize) -> Vec<usize> {
        let num_keys = 64;
        let lookup_range = 1_000;
        let mut keys = vec![];

        let chunk_size = 10_000;
        for chunk_start in (0..).step_by(chunk_size) {
            let triplet = Arc::new(Mutex::new(Vec::new()));
            let quintet = Arc::new(Mutex::new(Vec::from_iter((0..16).map(|_| BTreeSet::new()))));
            (chunk_start..chunk_start + chunk_size + lookup_range)
                .into_par_iter()
                .for_each(|idx| {
                    let message = self.salt.clone() + &idx.to_string();
                    let mut digest = md5::digest(message.as_bytes());
                    for _ in 0..extra_hashes {
                        digest = md5::digest(&digest16_to_hex_char(digest));
                    }
                    let hex = digest16_to_hex_byte(digest);
                    if (idx < chunk_start + chunk_size)
                        && let Some(&[ch, _, _]) =
                            hex.array_windows().find(|[a, b, c]| all_eq!(a, b, c))
                    {
                        let mut t = triplet
                            .lock()
                            .unwrap_or_else(|err| panic!("Failed to obtain lock: {err}"));
                        t.push((idx, ch));
                    }
                    for &[ch, _, _, _, _] in hex
                        .array_windows()
                        .filter(|[a, b, c, d, e]| all_eq!(a, b, c, d, e))
                    {
                        let mut q = quintet
                            .lock()
                            .unwrap_or_else(|err| panic!("Failed to obtain lock: {err}"));
                        q[ch as usize].insert(idx);
                    }
                });

            // Check if we have enough keys yet
            let mut t = triplet
                .lock()
                .unwrap_or_else(|err| panic!("Failed to obtain lock: {err}"));

            for (idx, ch) in t.drain(..) {
                let q = quintet
                    .lock()
                    .unwrap_or_else(|err| panic!("Failed to obtain lock: {err}"));
                if (1..=lookup_range)
                    .into_par_iter()
                    .any(|i| q[ch as usize].contains(&(idx + i)))
                {
                    keys.push(idx);
                }
            }
            drop(t);

            if keys.len() >= num_keys {
                break;
            }
        }
        keys.sort_unstable();
        keys
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let salt = input.trim().to_string();
        Ok(Self { salt })
    }

    fn part1(&self) -> String {
        let keys = self.find_keys(0);
        keys[63].to_string()
    }

    fn part2(&self) -> String {
        let keys = self.find_keys(2016);
        keys[63].to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
