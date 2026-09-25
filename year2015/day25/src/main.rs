use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, usize},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
};
use util::{Solution, chore, parser};

struct Puzzle {
    row: usize,
    column: usize,
}

impl Puzzle {
    fn parse_coord(input: &str) -> IResult<&str, (usize, usize)> {
        preceded(
            many1(alt((alpha1, tag(" "), tag(","), tag(".")))),
            terminated(separated_pair(usize, tag(", column "), usize), char('.')),
        )
        .parse_complete(input)
    }

    const fn coord_to_index(row: usize, column: usize) -> usize {
        let height = row + column - 1;
        height * (height - 1) / 2 + column
    }

    const fn next_code(code: u64) -> u64 {
        (code * 25_25_33) % 33_55_43_93
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (row, column) = parser::parse_input_str(input, Self::parse_coord)?;
        Ok(Self { row, column })
    }

    fn part1(&self) -> String {
        #[allow(clippy::inconsistent_digit_grouping)]
        let mut code = 2015_11_25;
        for _ in 1..Self::coord_to_index(self.row, self.column) {
            code = Self::next_code(code);
        }
        code.to_string()
    }

    fn part2(&self) -> String {
        String::default()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
