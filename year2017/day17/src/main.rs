use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    step_size: usize,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let step_size = input.trim().parse()?;
        Ok(Self { step_size })
    }

    fn part1(&self) -> String {
        let mut buffer = Vec::with_capacity(2018);
        let mut pos = 0;
        buffer.push(0);
        for val in 1..=2017 {
            pos += self.step_size;
            pos %= buffer.len();
            pos += 1;
            buffer.insert(pos, val);
        }
        buffer[(pos + 1) % buffer.len()].to_string()
    }

    fn part2(&self) -> String {
        let mut next_val = 0;
        let mut pos = 0;
        for len in 1..=50_000_000 {
            pos += self.step_size;
            pos %= len;
            // Only track what is after 0
            if pos == 0 {
                next_val = len;
            }
            pos += 1;
        }
        next_val.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
