use std::collections::BTreeMap;

use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::{
        char,
        complete::{alpha1, i16, line_ending},
    },
    combinator::{map, value},
    multi::separated_list1,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Puzzle {
    preferences: Array2<i16>,
}

impl Puzzle {
    fn parse_line(input: &str) -> IResult<&str, (&str, i16, &str)> {
        map(
            (
                alpha1,
                tag(" would "),
                alt((value(true, tag("gain ")), value(false, tag("lose ")))),
                i16,
                tag(" happiness units by sitting next to "),
                alpha1,
                char('.'),
            ),
            |(name, _, gain, val, _, other, _)| (name, if gain { val } else { -val }, other),
        )
        .parse_complete(input)
    }

    // Unlike day 9, this time it can be either open-loop or closed-loop.
    // You CAN still use dynamic programming for solution, but it would make it
    // unneccessarily complicated to handle both cases at once.
    fn brute_force_all_arrangement(&self, include_self: bool) -> i16 {
        let n = self.preferences.ncols();
        (0..n)
            .permutations(n)
            .par_bridge()
            .map(|mut ordering| {
                if !include_self {
                    ordering.push(ordering[0]);
                }
                ordering
                    .array_windows()
                    .map(|&neighbors| self.preferences[<(_, _)>::from(neighbors)])
                    .sum::<i16>()
            })
            .max()
            .unwrap_or_else(|| unreachable!("Input must have an optimal solution"))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let paired_preferences =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_line))?;
        let name_idx = paired_preferences
            .iter()
            .flat_map(|&(this, _, other)| [this, other])
            .fold(BTreeMap::new(), |mut acc, x| {
                let len = acc.len();
                acc.entry(x).or_insert_with(|| len);
                acc
            });
        let n = name_idx.len();
        let paired_preferences = paired_preferences
            .into_iter()
            .map(|(this, change, other)| {
                (
                    (
                        *name_idx
                            .get(this)
                            .unwrap_or_else(|| unreachable!("All names must be present in index")),
                        *name_idx
                            .get(other)
                            .unwrap_or_else(|| unreachable!("All names must be present in index")),
                    ),
                    change,
                )
            })
            .collect::<BTreeMap<_, _>>();
        let preferences = Array2::from_shape_fn((n, n), |(i, j)| {
            if i == j {
                return 0;
            }
            *paired_preferences.get(&(i, j)).unwrap_or_else(|| {
                panic!("Given pair doesn't exist in input: {i} to {j} ({name_idx:?})")
            }) + *paired_preferences.get(&(j, i)).unwrap_or_else(|| {
                panic!("Given pair doesn't exist in input: {i} to {j} ({name_idx:?})")
            })
        });
        Ok(Self { preferences })
    }

    fn part1(&self) -> String {
        self.brute_force_all_arrangement(false).to_string()
    }

    fn part2(&self) -> String {
        self.brute_force_all_arrangement(true).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
