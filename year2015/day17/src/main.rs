use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{line_ending, u8},
    multi::separated_list1,
};
use util::{Solution, chore, parser};

struct Puzzle {
    containers: Vec<u8>,
    volume: u8,
}

impl Puzzle {
    fn parse_containers(input: &str) -> IResult<&str, Vec<u8>> {
        separated_list1(line_ending, u8).parse_complete(input)
    }

    fn dp<T>(&self, init: T, modify: fn(&mut T, &T), insert: fn(T) -> T, eval: fn(&T) -> u32) -> u32
    where
        T: Clone,
    {
        let mut dp = BTreeMap::from_iter([(0, init)]);
        for container in &self.containers {
            #[allow(
                clippy::needless_collect,
                reason = "we will take a mutable reference of dp in the loop, so we need to obtain the list of values first"
            )]
            let previous = dp
                .keys()
                .copied()
                .filter(|&v| v + container <= self.volume)
                .collect::<Vec<_>>();
            for v in previous.into_iter().rev() {
                let last = dp
                    .get(&v)
                    .unwrap_or_else(|| panic!("{v} must be in dp"))
                    .clone();
                dp.entry(container + v)
                    .and_modify(|cnt| modify(cnt, &last))
                    .or_insert_with(|| insert(last));
            }
        }
        dp.get(&self.volume).map(eval).unwrap_or_default()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let containers = parser::parse_input_str(input, Self::parse_containers)?;
        let volume = if E { 25 } else { 150 };
        Ok(Self { containers, volume })
    }

    fn part1(&self) -> String {
        self.dp(1, |acc, x| *acc += x, |x| x, Clone::clone)
            .to_string()
    }

    fn part2(&self) -> String {
        self.dp(
            BTreeMap::from_iter([(0, 1)]),
            |acc, x| {
                for (k, v) in x {
                    acc.entry(k + 1).and_modify(|cnt| *cnt += v).or_insert(*v);
                }
            },
            |counts| counts.into_iter().map(|(k, v)| (k + 1, v)).collect(),
            |counts| {
                counts
                    .iter()
                    .min_by_key(|&(n, _)| n)
                    .map(|(_, cnt)| *cnt)
                    .unwrap_or_default()
            },
        )
        .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
