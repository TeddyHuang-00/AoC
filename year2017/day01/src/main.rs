use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::{many1, many1_count},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Count {
    digit: u8,
    count: u8,
}

struct Puzzle {
    counts: Vec<Count>,
}

impl Puzzle {
    #[allow(clippy::cast_possible_truncation)]
    fn parse_sequence(input: &str) -> IResult<&str, Vec<Count>> {
        many1(alt((
            map(many1_count(char('1')), |count| Count {
                digit: 1,
                count: count as u8,
            }),
            map(many1_count(char('2')), |count| Count {
                digit: 2,
                count: count as u8,
            }),
            map(many1_count(char('3')), |count| Count {
                digit: 3,
                count: count as u8,
            }),
            map(many1_count(char('4')), |count| Count {
                digit: 4,
                count: count as u8,
            }),
            map(many1_count(char('5')), |count| Count {
                digit: 5,
                count: count as u8,
            }),
            map(many1_count(char('6')), |count| Count {
                digit: 6,
                count: count as u8,
            }),
            map(many1_count(char('7')), |count| Count {
                digit: 7,
                count: count as u8,
            }),
            map(many1_count(char('8')), |count| Count {
                digit: 8,
                count: count as u8,
            }),
            map(many1_count(char('9')), |count| Count {
                digit: 9,
                count: count as u8,
            }),
        )))
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let counts = parser::parse_input_str(input, Self::parse_sequence)?;
        Ok(Self { counts })
    }

    fn part1(&self) -> String {
        let mut sum = self
            .counts
            .iter()
            .copied()
            .map(|Count { digit, count }| u16::from(digit) * u16::from(count - 1))
            .sum::<u16>();
        if self.counts.first().map(|&Count { digit, count: _ }| digit)
            == self.counts.last().map(|&Count { digit, count: _ }| digit)
        {
            // The single segment forms a circular loop
            sum += u16::from(self.counts[0].digit);
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let digits =
            self.counts
                .iter()
                .copied()
                .fold(Vec::new(), |mut acc, Count { digit, count }| {
                    acc.extend_from_slice(&vec![digit; count as usize]);
                    acc
                });
        let half = digits.len() / 2;
        (0..half)
            .fold(0, |mut acc, idx| {
                if digits[idx] == digits[idx + half] {
                    acc += 2 * u16::from(digits[idx]);
                }
                acc
            })
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
