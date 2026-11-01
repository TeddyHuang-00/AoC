use anyhow::Result;
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::char,
    combinator::value, multi::separated_list1,
};
use util::{Solution, chore, parser, vector::Vector2D};

#[derive(Clone, Copy)]
enum Hexagon {
    N,
    NW,
    SW,
    S,
    SE,
    NE,
}

impl Hexagon {
    /// The third axis is implicitly modeled as `-(x + y)`
    const fn as_vec2d(self) -> Vector2D<isize> {
        match self {
            Self::N => Vector2D { x: 1, y: 0 },
            Self::NW => Vector2D { x: 1, y: -1 },
            Self::SW => Vector2D { x: 0, y: -1 },
            Self::S => Vector2D { x: -1, y: 0 },
            Self::SE => Vector2D { x: -1, y: 1 },
            Self::NE => Vector2D { x: 0, y: 1 },
        }
    }
}

struct Puzzle {
    path: Vec<Hexagon>,
}

impl Puzzle {
    fn parse_hexagon(input: &str) -> IResult<&str, Hexagon> {
        alt((
            value(Hexagon::NW, tag("nw")),
            value(Hexagon::NE, tag("ne")),
            value(Hexagon::N, tag("n")),
            value(Hexagon::SW, tag("sw")),
            value(Hexagon::SE, tag("se")),
            value(Hexagon::S, tag("s")),
        ))
        .parse_complete(input)
    }

    fn distance(Vector2D { x, y }: Vector2D<isize>) -> usize {
        let z = -(x + y);
        // The distance would be half of the Manhatten distance in imaginary 3D
        // grid. Given that `x + y + z = 0`, it will just be the maximum of
        // them, i.e., `(|x| + |y| + |z|)/2 === max(|x|, |y|, |z|)`
        x.unsigned_abs().max(y.unsigned_abs()).max(z.unsigned_abs())
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let path = parser::parse_input_str(input, separated_list1(char(','), Self::parse_hexagon))?;
        Ok(Self { path })
    }

    fn part1(&self) -> String {
        let coord: Vector2D<isize> = self
            .path
            .iter()
            .copied()
            .fold(Vector2D::default(), |pos, x| pos + x.as_vec2d());
        Self::distance(coord).to_string()
    }

    fn part2(&self) -> String {
        let (max, _) =
            self.path
                .iter()
                .copied()
                .fold((0, Vector2D::default()), |(mut max, mut pos), x| {
                    pos += x.as_vec2d();
                    max = max.max(Self::distance(pos));
                    (max, pos)
                });
        max.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
