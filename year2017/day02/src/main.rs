use std::collections::BTreeSet;

use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{line_ending, space1, u16},
    multi::separated_list1,
};
use util::{Solution, chore, parser};

struct Puzzle {
    spreadsheet: Vec<Vec<u16>>,
}

impl Puzzle {
    fn parse_row(input: &str) -> IResult<&str, Vec<u16>> {
        separated_list1(space1, u16).parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let spreadsheet =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_row))?;
        Ok(Self { spreadsheet })
    }

    fn part1(&self) -> String {
        self.spreadsheet
            .iter()
            .map(|row| {
                let (min, max) =
                    row.iter()
                        .copied()
                        .fold((u16::MAX, u16::MIN), |(mut min, mut max), num| {
                            min = min.min(num);
                            max = max.max(num);
                            (min, max)
                        });
                max - min
            })
            .sum::<u16>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.spreadsheet
            .iter()
            .map(|row| {
                let mut numbers = BTreeSet::new();
                for num in row {
                    for other in &numbers {
                        let dividend = num.max(other);
                        let divisor = num.min(other);
                        if dividend % divisor == 0 {
                            return dividend / divisor;
                        }
                    }
                    numbers.insert(*num);
                }
                panic!("No divisible pair found")
            })
            .sum::<u16>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
