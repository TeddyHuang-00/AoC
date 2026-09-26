use std::{cmp::Reverse, collections::BTreeMap, convert::identity};

use anyhow::Result;
use ndarray::{Zip, parallel::prelude::*, prelude::*};
use nom::{
    IResult, Parser,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
};
use util::{Solution, chore, parser};

struct Puzzle {
    char_grid: Array2<char>,
}

impl Puzzle {
    fn parse_code(input: &str) -> IResult<&str, Vec<char>> {
        map(alpha1, |s: &str| s.chars().collect()).parse_complete(input)
    }

    fn decode<T>(&self, key: fn(u8) -> T) -> String
    where
        T: Ord,
    {
        Zip::from(self.char_grid.axis_iter(Axis(1)))
            .into_par_iter()
            .map(|(chars,)| {
                chars
                    .iter()
                    .fold(BTreeMap::new(), |mut acc, &ch| {
                        acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1u8);
                        acc
                    })
                    .into_iter()
                    .max_by_key(|&(_, cnt)| key(cnt))
                    .map_or_else(|| panic!("Input is empty"), |(ch, _)| ch)
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let chars = parser::parse_input_str(input, separated_list1(line_ending, Self::parse_code))?;
        let char_grid = parser::nested_vec_to_array2(chars)?;
        Ok(Self { char_grid })
    }

    fn part1(&self) -> String {
        self.decode(identity)
    }

    fn part2(&self) -> String {
        self.decode(Reverse)
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
