use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Puzzle {
    dims: Vec<[u8; 3]>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let dims = parser::parse_lines(input.trim(), |line| {
            let parts: Vec<u8> = line
                .split('x')
                .map(|s| s.parse::<u8>().map_err(|e| anyhow::anyhow!(e)))
                .collect::<Result<Vec<u8>>>()?;
            if parts.len() != 3 {
                return Err(anyhow::anyhow!(
                    "Expected 3 dimensions, got {}: {line}",
                    parts.len()
                ));
            }
            Ok([parts[0], parts[1], parts[2]])
        })?;
        Ok(Self { dims })
    }

    fn part1(&self) -> String {
        self.dims
            .par_iter()
            .copied()
            .map(|[l, w, h]| {
                let [l, w, h] = [u32::from(l), u32::from(w), u32::from(h)];
                let side = [l * w, w * h, l * h];
                2 * (side[0] + side[1] + side[2])
                    + *side
                        .iter()
                        .min()
                        .unwrap_or_else(|| panic!("List should be none empty: {side:?}"))
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.dims
            .par_iter()
            .copied()
            .map(|[l, w, h]| {
                let [l, w, h] = [u32::from(l), u32::from(w), u32::from(h)];
                let longest = l.max(w).max(h);
                2 * (l + w + h - longest) + l * w * h
            })
            .sum::<u32>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
