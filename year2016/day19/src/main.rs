use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    num_elves: usize,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let num_elves = input.trim().parse()?;
        Ok(Self { num_elves })
    }

    /// I haven't really figured out the math behind this yet, but I do find
    /// this pattern from N=1..17. Probably because every power of two leads to
    /// an extra round of elimination.
    fn part1(&self) -> String {
        let n = self.num_elves;
        let k = n.next_power_of_two().ilog2() as usize - 1;
        let base = (0..k).map(|i| (1 << i) * (i + 1)).sum::<usize>();
        let residual = (n - (1 << k)) * (k + 1);
        let result = match (base + residual) % n {
            0 => n,
            r => r,
        };
        result.to_string()
    }

    /// Again, I just test the pattern for N=1..100, and then use what I found
    /// to generalize to any N.
    fn part2(&self) -> String {
        let n = self.num_elves;
        let mut k = 0;
        while 2 * 3usize.pow(k + 1) < n {
            k += 1;
        }
        let base = 3usize.pow(k) * (2 * k as usize + 1);
        let residual = (n - 2 * 3usize.pow(k)) * (k as usize + 2);
        let result = match (base + residual) % n {
            0 => n,
            r => r,
        };
        result.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
