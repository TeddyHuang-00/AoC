use std::{
    collections::{BTreeSet, HashMap},
    hash::BuildHasherDefault,
};

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{map, value},
    multi::{many_m_n, many1, separated_list1},
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, hasher::FastHasher, parser};

#[derive(Clone)]
struct Pots {
    min: i64,
    container: Vec<bool>,
}

impl Pots {
    fn from_range((left, right): (i64, i64)) -> Self {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "The list should be very short"
        )]
        Self {
            min: left - 4,
            container: vec![false; right.abs_diff(left) as usize + 9],
        }
    }

    fn range(&self) -> (i64, i64) {
        let left = self
            .container
            .iter()
            .position(|&pot| pot)
            .map_or_else(|| self.min.unsigned_abs(), |pos| pos as u64)
            .cast_signed();
        let right = self
            .container
            .iter()
            .rposition(|&pot| pot)
            .map_or_else(|| self.min.unsigned_abs(), |pos| pos as u64)
            .cast_signed();
        (left + self.min, right + self.min)
    }

    fn get_mut(&mut self, index: i64) -> &mut bool {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "The list should be very short"
        )]
        &mut self.container[index.abs_diff(self.min) as usize]
    }

    fn next(&self, rules: &BTreeSet<[bool; 5]>) -> Self {
        let mut next = Self::from_range(self.range());
        for (idx, neighbors) in self
            .container
            .array_windows()
            .enumerate()
            .map(|(idx, win)| (self.min.strict_add_unsigned(idx as u64 + 2), win))
        {
            if rules.contains(neighbors) {
                *next.get_mut(idx) = true;
            }
        }
        next
    }

    fn sum(&self) -> i64 {
        self.container
            .iter()
            .enumerate()
            .filter_map(|(idx, pot)| pot.then_some(self.min.strict_add_unsigned(idx as u64)))
            .sum()
    }

    fn hash_key(&self) -> Vec<u64> {
        let (start, end) = self.range();
        #[allow(
            clippy::cast_possible_truncation,
            reason = "The list should be very short"
        )]
        let (start, end) = (
            start.abs_diff(self.min) as usize,
            end.abs_diff(self.min) as usize,
        );
        self.container[start..=end]
            .chunks(64)
            .map(|chunk| {
                chunk.iter().fold(0, |mut acc, &x| {
                    acc <<= 1;
                    acc |= u64::from(x);
                    acc
                })
            })
            .collect()
    }
}

struct Puzzle {
    initial_state: Pots,
    rules: BTreeSet<[bool; 5]>,
}

impl Puzzle {
    fn parse_plant(input: &str) -> IResult<&str, bool> {
        alt((value(true, char('#')), value(false, char('.')))).parse_complete(input)
    }

    fn parse_initial_state(input: &str) -> IResult<&str, Pots> {
        map(
            preceded(tag("initial state: "), many1(Self::parse_plant)),
            |init| {
                // Pad both ends
                let mut container = Vec::with_capacity(init.len() + 8);
                container.extend_from_slice(&[false; 4]);
                container.extend(init);
                container.extend(&[false; 4]);
                Pots { min: -4, container }
            },
        )
        .parse_complete(input)
    }

    fn parse_rule(input: &str) -> IResult<&str, ([bool; 5], bool)> {
        map(
            separated_pair(
                many_m_n(5, 5, Self::parse_plant),
                tag(" => "),
                Self::parse_plant,
            ),
            |(neighbor, next)| {
                (
                    neighbor
                        .as_array()
                        .unwrap_or_else(|| unreachable!())
                        .to_owned(),
                    next,
                )
            },
        )
        .parse_complete(input)
    }

    fn parse_input(input: &str) -> IResult<&str, (Pots, BTreeSet<[bool; 5]>)> {
        separated_pair(
            Self::parse_initial_state,
            (line_ending, line_ending),
            map(separated_list1(line_ending, Self::parse_rule), |rules| {
                rules
                    .into_iter()
                    .filter_map(|(pattern, next)| next.then_some(pattern))
                    .collect()
            }),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (initial_state, rules) = parser::parse_input_str(input, Self::parse_input)?;
        Ok(Self {
            initial_state,
            rules,
        })
    }

    fn part1(&self) -> String {
        let mut pots = self.initial_state.clone();
        for _ in 0..20 {
            pots = pots.next(&self.rules);
        }
        pots.sum().to_string()
    }

    fn part2(&self) -> String {
        let mut pots = self.initial_state.clone();
        let mut seen = HashMap::with_hasher(BuildHasherDefault::<FastHasher>::new());
        seen.insert(pots.hash_key(), (0, pots.range().0));
        let total_steps = 50_000_000_000;
        for iter in 1..=total_steps {
            pots = pots.next(&self.rules);
            let (left, _) = pots.range();
            if let Some((last_iter, last_left)) = seen.insert(pots.hash_key(), (iter, left)) {
                let delta = left - last_left;
                let period = iter - last_iter;
                let num_cycles = (total_steps - iter) / period;
                pots.min += delta * num_cycles;
                let remaining_cycles = (total_steps - iter) % period;
                for _ in 0..remaining_cycles {
                    pots = pots.next(&self.rules);
                }
                break;
            }
        }
        pots.sum().to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
