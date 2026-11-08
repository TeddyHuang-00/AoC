use anyhow::Result;
use ndarray::{parallel::prelude::*, prelude::*};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u16},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser, vector::Vector2D};

#[derive(Clone, Copy)]
struct Claim {
    margin: Vector2D<u16>,
    dimension: Vector2D<u16>,
}

struct Puzzle {
    claims: Vec<Claim>,
    fabric: Array2<u16>,
}

impl Puzzle {
    fn parse_claim(input: &str) -> IResult<&str, Claim> {
        map(
            separated_pair(
                preceded(
                    (tag("#"), u16, tag(" @ ")),
                    separated_pair(u16, tag(","), u16),
                ),
                tag(": "),
                separated_pair(u16, tag("x"), u16),
            ),
            |((mx, my), (w, h))| Claim {
                margin: Vector2D::new(mx, my),
                dimension: Vector2D::new(w, h),
            },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let claims =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_claim))?;
        let fabric = claims.iter().fold(
            Array2::zeros((1_000, 1_000)),
            |mut fab, &Claim { margin, dimension }| {
                fab.slice_mut(s![
                    margin.y as usize..(margin.y + dimension.y) as usize,
                    margin.x as usize..(margin.x + dimension.x) as usize
                ])
                .mapv_inplace(|cnt: u16| cnt + 1);
                fab
            },
        );
        Ok(Self { claims, fabric })
    }

    fn part1(&self) -> String {
        self.fabric
            .into_par_iter()
            .filter(|&&cnt| cnt > 1)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        for (id, &Claim { margin, dimension }) in self.claims.iter().enumerate() {
            if self
                .fabric
                .slice(s![
                    margin.y as usize..(margin.y + dimension.y) as usize,
                    margin.x as usize..(margin.x + dimension.x) as usize
                ])
                .iter()
                .all(|&cnt| cnt == 1)
            {
                return (id + 1).to_string();
            }
        }
        panic!("No solution found")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
