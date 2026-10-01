use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Range(u32, u32);

struct Puzzle {
    ranges: Vec<Range>,
}

impl Puzzle {
    fn parse_range(input: &str) -> IResult<&str, Range> {
        map(separated_pair(u32, char('-'), u32), |(s, e)| Range(s, e)).parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let mut raw =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_range))?;
        raw.sort_unstable_by_key(|r| r.0);
        let mut ranges: Vec<Range> = Vec::with_capacity(raw.len());
        for range in raw {
            if let Some(last) = ranges.last_mut()
                && last.1 >= range.0
            {
                last.1 = last.1.max(range.1);
            } else {
                ranges.push(range);
            }
        }
        Ok(Self { ranges })
    }

    fn part1(&self) -> String {
        let mut ip = 0;
        for range in &self.ranges {
            if ip < range.0 {
                return ip.to_string();
            }
            ip = range.1 + 1;
        }
        ip.to_string()
    }

    fn part2(&self) -> String {
        0u32.wrapping_sub(self.ranges.iter().map(|Range(s, e)| e - s + 1).sum())
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
