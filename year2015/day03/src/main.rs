use std::collections::BTreeSet;

use anyhow::Result;
use util::{Solution, chore, parser, vector::Vector2D};

type Position = Vector2D<i32>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn to_vector(self) -> Position {
        match self {
            Self::Up => Position::new(0, 1),
            Self::Down => Position::new(0, -1),
            Self::Left => Position::new(-1, 0),
            Self::Right => Position::new(1, 0),
        }
    }
}

struct Puzzle {
    directions: Vec<Direction>,
}

impl Puzzle {
    fn traverse<I>(directions: I) -> BTreeSet<Position>
    where
        I: Iterator<Item = Direction>,
    {
        directions
            .into_iter()
            .scan(Position::default(), |pos, dir| {
                *pos += dir.to_vector();
                Some(*pos)
            })
            .chain(std::iter::once(Position::default()))
            .collect::<BTreeSet<_>>()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let directions = parser::parse_chars(input.trim(), |ch| {
            Ok(match ch {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => anyhow::bail!("Invalid character: {ch}"),
            })
        })?;
        Ok(Self { directions })
    }

    fn part1(&self) -> String {
        Self::traverse(self.directions.iter().copied())
            .len()
            .to_string()
    }

    fn part2(&self) -> String {
        let (left, right): (Vec<_>, Vec<_>) = self
            .directions
            .iter()
            .copied()
            .enumerate()
            .partition(|(idx, _)| idx % 2 == 0);
        Self::traverse(left.into_iter().map(|(_, dir)| dir))
            .union(&Self::traverse(right.into_iter().map(|(_, dir)| dir)))
            .collect::<BTreeSet<_>>()
            .len()
            .to_string()
    }
}

chore!(env!("CARGO_PKG_NAME"));
