use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{line_ending, space1, u16},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Triangle(u16, u16, u16);

impl Triangle {
    fn is_valid(self) -> bool {
        let max = self.0.max(self.1).max(self.2);
        let rest = self.0 + self.1 + self.2 - max;
        rest > max
    }
}

struct Puzzle {
    triangles: Vec<Triangle>,
}

impl Puzzle {
    fn parse_triangle(input: &str) -> IResult<&str, Triangle> {
        map(
            (
                preceded(space1, u16),
                preceded(space1, u16),
                preceded(space1, u16),
            ),
            |(a, b, c)| Triangle(a, b, c),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let triangles =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_triangle))?;
        Ok(Self { triangles })
    }

    fn part1(&self) -> String {
        self.triangles
            .iter()
            .filter(|t| t.is_valid())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.triangles
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| {
                let (a, b, c) = (t[0], t[1], t[2]);
                [
                    Triangle(a.0, b.0, c.0).is_valid(),
                    Triangle(a.1, b.1, c.1).is_valid(),
                    Triangle(a.2, b.2, c.2).is_valid(),
                ]
                .into_iter()
                .filter(Clone::clone)
                .count()
            })
            .sum::<usize>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
