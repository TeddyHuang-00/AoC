use std::collections::BTreeSet;

use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

type CharSet = [u8; 26];

struct Puzzle {
    passphrases: Vec<Vec<&'static str>>,
}

impl Puzzle {
    fn parse_passphrase(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(space1, alpha1).parse(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let passphrases =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_passphrase))?;
        Ok(Self { passphrases })
    }

    fn part1(&self) -> String {
        self.passphrases
            .par_iter()
            .filter(|words| {
                words
                    .iter()
                    .try_fold(BTreeSet::new(), |mut acc, &word| {
                        if acc.insert(word) { Some(acc) } else { None }
                    })
                    .is_some()
            })
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.passphrases
            .par_iter()
            .filter(|words| {
                words
                    .iter()
                    .try_fold(BTreeSet::new(), |mut acc, &word| {
                        let cnt = word
                            .as_bytes()
                            .iter()
                            .fold(CharSet::default(), |mut acc, &c| {
                                acc[(c - b'a') as usize] += 1;
                                acc
                            });
                        if acc.insert(cnt) { Some(acc) } else { None }
                    })
                    .is_some()
            })
            .count()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
