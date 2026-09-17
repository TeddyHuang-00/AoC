use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        char,
        complete::{alphanumeric1, line_ending},
    },
    combinator::value,
    multi::{fold_many0, separated_list1},
    sequence::delimited,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Escape {
    Backslash,
    Quote,
    Hex,
}

struct Puzzle {
    line: usize,
    backslash: usize,
    quote: usize,
    byte: usize,
}

impl Puzzle {
    fn parse_escaped(input: &str) -> IResult<&str, BTreeMap<Escape, usize>> {
        delimited(
            char('"'),
            fold_many0(
                alt((
                    // Ignore normal characters, we only care about escape sequences
                    value(None, alphanumeric1),
                    value(Some(Escape::Backslash), tag("\\\\")),
                    value(Some(Escape::Quote), tag("\\\"")),
                    value(Some(Escape::Hex), tag("\\x")),
                )),
                BTreeMap::new,
                |mut acc, item| {
                    if let Some(escape) = item {
                        acc.entry(escape)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    acc
                },
            ),
            char('"'),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let counts =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_escaped))?;
        let line_count = counts.len();
        let counts = counts.into_iter().fold(BTreeMap::new(), |mut acc, map| {
            for (escape, count) in map {
                acc.entry(escape)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
            acc
        });

        Ok(Self {
            line: line_count,
            backslash: *counts.get(&Escape::Backslash).unwrap_or(&0),
            quote: *counts.get(&Escape::Quote).unwrap_or(&0),
            byte: *counts.get(&Escape::Hex).unwrap_or(&0),
        })
    }

    fn part1(&self) -> String {
        (2 * self.line + self.backslash + self.quote + 3 * self.byte).to_string()
    }

    fn part2(&self) -> String {
        (4 * self.line + 2 * self.backslash + 2 * self.quote + self.byte).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
