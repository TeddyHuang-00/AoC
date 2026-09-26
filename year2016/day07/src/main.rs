use std::collections::BTreeSet;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{alpha1, char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

enum Segment {
    Super(&'static str),
    Hyper(&'static str),
}

struct Address(Vec<Segment>);

struct Puzzle {
    addresses: Vec<Address>,
}

impl Puzzle {
    fn parse_ip(input: &'static str) -> IResult<&'static str, Address> {
        map(
            many1(alt((
                map(alpha1, Segment::Super),
                map(delimited(char('['), alpha1, char(']')), Segment::Hyper),
            ))),
            Address,
        )
        .parse_complete(input)
    }

    fn has_abba(segment: &str) -> bool {
        segment
            .as_bytes()
            .array_windows()
            .any(|[a, b, c, d]| a == d && b == c && a != b)
    }

    fn find_aba(segment: &str) -> Vec<(u8, u8)> {
        segment
            .as_bytes()
            .array_windows()
            .filter_map(|&[a, b, c]| (a == c && a != b).then_some((a, b)))
            .collect()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let addresses =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_ip))?;
        Ok(Self { addresses })
    }

    fn part1(&self) -> String {
        self.addresses
            .par_iter()
            .filter(|addr| {
                let mut has_abba_in_normal = false;
                for segment in &addr.0 {
                    match segment {
                        Segment::Super(s) => {
                            if !has_abba_in_normal && Self::has_abba(s) {
                                has_abba_in_normal = true;
                            }
                        }
                        Segment::Hyper(s) => {
                            if Self::has_abba(s) {
                                return false;
                            }
                        }
                    }
                }
                has_abba_in_normal
            })
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.addresses
            .par_iter()
            .filter(|addr| {
                let mut aba = BTreeSet::new();
                for segment in addr.0.iter().filter_map(|segment| {
                    if let Segment::Super(s) = segment {
                        Some(s)
                    } else {
                        None
                    }
                }) {
                    aba.extend(Self::find_aba(segment));
                }
                for segment in addr.0.iter().filter_map(|segment| {
                    if let Segment::Hyper(s) = segment {
                        Some(s)
                    } else {
                        None
                    }
                }) {
                    if Self::find_aba(segment)
                        .into_iter()
                        .any(|(a, b)| aba.contains(&(b, a)))
                    {
                        return true;
                    }
                }
                false
            })
            .count()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
