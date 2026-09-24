use anyhow::Result;
use rayon::prelude::*;
use util::{Solution, chore};

struct Puzzle {
    threshold: usize,
}

impl Puzzle {
    fn factor_decomposition(number: usize, limit: Option<usize>) -> usize {
        (1..=number)
            .take_while(|i| i * i <= number)
            .filter(|i| number.is_multiple_of(*i))
            .flat_map(|i| {
                if i * i == number {
                    vec![i]
                } else {
                    vec![i, number / i]
                }
            })
            .filter(|i| limit.is_none_or(|l| i * l >= number))
            .sum()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let threshold = input.trim().parse()?;
        Ok(Self { threshold })
    }

    fn part1(&self) -> String {
        (1..self.threshold / 10)
            .into_par_iter()
            .find_first(|&num| Self::factor_decomposition(num, None) * 10 >= self.threshold)
            .unwrap_or_else(|| unreachable!("Must be a valid solution"))
            .to_string()
    }

    fn part2(&self) -> String {
        (1..self.threshold / 10)
            .into_par_iter()
            .find_first(|&num| Self::factor_decomposition(num, Some(50)) * 11 >= self.threshold)
            .unwrap_or_else(|| unreachable!("Must be a valid solution"))
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
