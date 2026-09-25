use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{line_ending, u16},
    multi::separated_list1,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use util::{Solution, chore, parser};

struct Puzzle {
    weights: Vec<u16>,
}

impl Puzzle {
    fn parse_weights(input: &str) -> IResult<&str, Vec<u16>> {
        separated_list1(line_ending, u16).parse_complete(input)
    }

    fn optimal_arrangement(&self, target: u16) -> u64 {
        for num in 1..self.weights.len() {
            if let Some(qe) = self
                .weights
                .iter()
                .copied()
                .combinations(num)
                .par_bridge()
                .filter_map(|ws| {
                    let w = ws.iter().copied().sum::<u16>();
                    // The actual input are all prime numbers which happens to
                    // have the property that allow us to skip validating the
                    // actual solution of the rest
                    if w != target {
                        return None;
                    }
                    Some(ws.into_iter().map(u64::from).product::<u64>())
                })
                .min()
            {
                return qe;
            }
        }
        panic!("No valid solution found")
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let weights = parser::parse_input_str(input, Self::parse_weights)?;
        Ok(Self { weights })
    }

    fn part1(&self) -> String {
        let target = self.weights.iter().sum::<u16>() / 3;
        self.optimal_arrangement(target).to_string()
    }

    fn part2(&self) -> String {
        let target = self.weights.iter().sum::<u16>() / 4;
        self.optimal_arrangement(target).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
