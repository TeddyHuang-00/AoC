use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u8},
    combinator::map,
    multi::separated_list1,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Reindeer {
    max_speed: u8,
    fly_time: u8,
    rest_time: u8,
}

impl Reindeer {
    fn distance_at_time(self, seconds: u16) -> u16 {
        let cycle_time = u16::from(self.fly_time + self.rest_time);
        let (cycles, remaining) = (seconds / cycle_time, seconds % cycle_time);
        (cycles * u16::from(self.fly_time) + u16::from(self.fly_time).min(remaining))
            * u16::from(self.max_speed)
    }
}

struct Puzzle {
    reindeers: Vec<Reindeer>,
}

impl Puzzle {
    fn parse_line(input: &str) -> IResult<&str, Reindeer> {
        map(
            (
                alpha1,
                tag(" can fly "),
                u8,
                tag(" km/s for "),
                u8,
                tag(" seconds, but then must rest for "),
                u8,
                tag(" seconds."),
            ),
            |(_, _, max_speed, _, fly_time, _, rest_time, _)| Reindeer {
                max_speed,
                fly_time,
                rest_time,
            },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let reindeers =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_line))?;
        Ok(Self { reindeers })
    }

    fn part1(&self) -> String {
        self.reindeers
            .iter()
            .map(|&reindeer| reindeer.distance_at_time(2503))
            .max()
            .unwrap_or_else(|| unreachable!("Must be at least one reindeer"))
            .to_string()
    }

    fn part2(&self) -> String {
        (1..=2503)
            .into_par_iter()
            .flat_map_iter(|seconds| {
                let distances = self
                    .reindeers
                    .iter()
                    .map(|reindeer| reindeer.distance_at_time(seconds))
                    .collect::<Vec<_>>();
                let max_distance = *distances
                    .iter()
                    .max()
                    .unwrap_or_else(|| unreachable!("Must be at least one reindeer"));
                distances
                    .into_iter()
                    .enumerate()
                    .filter_map(|(idx, distance)| (distance == max_distance).then_some(idx))
                    .collect::<Vec<_>>()
            })
            .fold(BTreeMap::new, |mut acc, winner| {
                acc.entry(winner)
                    .and_modify(|pts| *pts += 1)
                    .or_insert(1u16);
                acc
            })
            .reduce(Default::default, |mut acc, x| {
                for (k, v) in x {
                    acc.entry(k).and_modify(|pts| *pts += v).or_insert(v);
                }
                acc
            })
            .into_values()
            .max()
            .unwrap_or_else(|| unreachable!("Must be at least one reindeer"))
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
