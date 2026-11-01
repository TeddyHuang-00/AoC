use std::collections::VecDeque;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

const A: u64 = 16807;
const B: u64 = 48271;
const M: u64 = (1 << 31) - 1;
const MASK: u64 = 0xffff;

struct Puzzle {
    a: u64,
    b: u64,
}

impl Puzzle {
    fn parse_seed(input: &str) -> IResult<&str, u64> {
        preceded(
            (
                tag("Generator "),
                alt((tag("A"), tag("B"))),
                tag(" starts with "),
            ),
            u64,
        )
        .parse_complete(input)
    }

    fn parse_pair(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(Self::parse_seed, line_ending, Self::parse_seed).parse_complete(input)
    }

    /// Using property of Mersenne number to speed up calculation
    #[inline]
    const fn mul_mod(a: u64, b: u64) -> u64 {
        let prod = a * b;
        let rem = (prod & M) + (prod >> 31);
        let rem = (rem & M) + (rem >> 31);
        if rem >= M { rem - M } else { rem }
    }

    const fn generate(a: u64, b: u64) -> (u64, u64) {
        (Self::mul_mod(a, A), Self::mul_mod(b, B))
    }

    const fn judge(a: u64, b: u64) -> bool {
        a & MASK == b & MASK
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (a, b) = parser::parse_input_str(input, Self::parse_pair)?;
        Ok(Self { a, b })
    }

    fn part1(&self) -> String {
        let (mut a, mut b) = (self.a, self.b);
        let mut count = 0;
        for _ in 0..40_000_000 {
            (a, b) = Self::generate(a, b);
            if Self::judge(a, b) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let (mut a, mut b) = (self.a, self.b);
        let (mut queue_a, mut queue_b) = (VecDeque::new(), VecDeque::new());
        let mut count = 0;
        for _ in 0..40_000_000 {
            (a, b) = Self::generate(a, b);
            if a.trailing_zeros() >= 2 {
                queue_a.push_back(a);
            }
            if b.trailing_zeros() >= 3 {
                queue_b.push_back(b);
            }
            while !queue_a.is_empty() && !queue_b.is_empty() {
                let (Some(a), Some(b)) = (queue_a.pop_front(), queue_b.pop_front()) else {
                    unreachable!("Queues must not be empty")
                };
                if Self::judge(a, b) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
