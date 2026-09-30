use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Disc {
    index: u8,
    positions: u8,
    initial: u8,
}

struct Puzzle {
    discs: Vec<Disc>,
}

impl Puzzle {
    fn parse_disc(input: &str) -> IResult<&str, Disc> {
        map(
            (
                preceded(tag("Disc #"), u8),
                delimited(tag(" has "), u8, tag(" positions; ")),
                delimited(tag("at time=0, it is at position "), u8, tag(".")),
            ),
            |(index, positions, initial)| Disc {
                index,
                positions,
                initial,
            },
        )
        .parse_complete(input)
    }

    fn chinese_remainder_theorem(discs: &[Disc]) -> u32 {
        let m = discs
            .iter()
            .map(|d| u32::from(d.positions))
            .product::<u32>();
        discs
            .iter()
            .map(|d| {
                let modu = u32::from(d.positions);
                let mi = m / modu;
                let t = (1..modu)
                    .find(|&t| t * mi % modu == 1)
                    .unwrap_or_else(|| unreachable!("must exist a valid number"));
                u32::from((d.positions * 2 - d.index - d.initial) % d.positions) * t * mi
            })
            .sum::<u32>()
            % m
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let discs = parser::parse_input_str(input, separated_list1(line_ending, Self::parse_disc))?;
        Ok(Self { discs })
    }

    fn part1(&self) -> String {
        Self::chinese_remainder_theorem(&self.discs).to_string()
    }

    fn part2(&self) -> String {
        let mut discs = self.discs.clone();
        discs.push(Disc {
            index: self
                .discs
                .iter()
                .max_by_key(|d| d.index)
                .unwrap_or_else(|| unreachable!("discs must not be empty"))
                .index
                + 1,
            positions: 11,
            initial: 0,
        });
        Self::chinese_remainder_theorem(&discs).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
