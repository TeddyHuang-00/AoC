use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    multi::separated_list1,
    sequence::separated_pair,
};
use util::{Solution, chore, integer_as_key, parser};

#[derive(Clone, Copy)]
struct Filter {
    value: u32,
    cycle: u32,
}

impl Filter {
    const fn next(self) -> Self {
        Self {
            value: self.value + self.cycle,
            ..self
        }
    }
}

integer_as_key!(Filter, value);

struct Puzzle {
    scanners: Vec<(u8, u8)>,
}

impl Puzzle {
    fn parse_scanner(input: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, tag(": "), u8).parse_complete(input)
    }

    const fn greatest_common_divisor(mut a: u32, mut b: u32) -> u32 {
        if a < b {
            (a, b) = (b, a);
        }
        while b > 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    const fn least_common_multiple(a: u32, b: u32) -> u32 {
        a / Self::greatest_common_divisor(a, b) * b
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let scanners =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_scanner))?;
        Ok(Self { scanners })
    }

    fn part1(&self) -> String {
        self.scanners
            .iter()
            .filter_map(|&(depth, range)| {
                (depth % (2 * (range - 1)) == 0).then_some(u32::from(depth) * u32::from(range))
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let max_possible = self.scanners.iter().copied().fold(1, |acc, (_, range)| {
            Self::least_common_multiple(acc, 2 * (u32::from(range) - 1))
        });
        for delay in 0..max_possible {
            if self.scanners.iter().copied().all(|(depth, range)| {
                (delay + u32::from(depth)) % (2 * (u32::from(range) - 1)) != 0
            }) {
                return delay.to_string();
            }
        }
        let mut filters =
            BinaryHeap::from_iter(self.scanners.iter().copied().map(|(depth, range)| {
                let cycle = 2 * (range - 1);
                let value = (cycle - (depth % cycle)) % cycle;
                Reverse(Filter {
                    value: value.into(),
                    cycle: cycle.into(),
                })
            }));
        let mut delay = 0;
        while delay < max_possible {
            if let Some(Reverse(filter)) = filters.peek()
                && filter.value > delay
            {
                return delay.to_string();
            }
            while let Some(Reverse(filter)) = filters.peek()
                && filter.value == delay
            {
                let Some(Reverse(filter)) = filters.pop() else {
                    unreachable!("Heap will never be empty")
                };
                filters.push(Reverse(filter.next()));
            }
            delay += 1;
        }
        panic!("No valid solution")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
