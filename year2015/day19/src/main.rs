use std::{
    collections::{HashMap, HashSet},
    ops::Not,
};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use util::{Solution, chore, parser};

struct Puzzle {
    replacements: Vec<(&'static str, &'static str)>,
    molecule: &'static str,
}

impl Puzzle {
    #[allow(clippy::type_complexity)]
    fn parse_replacements_and_molecule(input: &str) -> IResult<&str, (Vec<(&str, &str)>, &str)> {
        separated_pair(
            separated_list1(
                line_ending,
                separated_pair(alphanumeric1, tag(" => "), alphanumeric1),
            ),
            (line_ending, line_ending),
            alphanumeric1,
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let (replacements, molecule) =
            parser::parse_input_str(input, Self::parse_replacements_and_molecule)?;
        Ok(Self {
            replacements,
            molecule,
        })
    }

    fn part1(&self) -> String {
        self.replacements
            .par_iter()
            .flat_map(|&(from, to)| {
                self.molecule
                    .match_indices(from)
                    .par_bridge()
                    .map(move |(idx, _)| {
                        self.molecule[..idx].to_string() + to + &self.molecule[(idx + from.len())..]
                    })
            })
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn part2(&self) -> String {
        let elements = {
            let breakpoints = self
                .molecule
                .match_indices(|ch: char| ch.is_ascii_uppercase())
                .map(|(idx, _)| idx)
                .chain(std::iter::once(self.molecule.len()))
                .collect::<Vec<_>>();
            breakpoints
                .array_windows()
                .map(|&[start, end]| &self.molecule[start..end])
                .fold(HashMap::new(), |mut acc, elem| {
                    acc.entry(elem).and_modify(|cnt| *cnt += 1).or_insert(1);
                    acc
                })
        };
        // Every rule is one of:
        // - X -> A B
        // - X -> A Rn B Ar
        // - X -> A Rn B Y C Ar
        // Because every rule increases the number of elements by exactly 1
        // (excepting Rn, Ar and Y; and the rule about e is special case but we
        // know it is a one-time occurrence), we can directly calculate the
        // exact number of steps without ever computing the actual path.
        (elements
            .iter()
            .filter_map(|(elem, &count)| ["Rn", "Ar", "Y"].contains(elem).not().then_some(count))
            .sum::<u16>()
            - 1
            - elements.get("Y").copied().unwrap_or_default())
        .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
