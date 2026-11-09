use std::collections::hash_map::Entry;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
};
use util::{Solution, chore, hasher::FxHashMap, parser, vector::Vector2D};

#[derive(Clone, Copy, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl From<Direction> for Vector2D<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self::new(-1, 0),
            Direction::Right => Self::new(0, 1),
            Direction::Down => Self::new(1, 0),
            Direction::Left => Self::new(0, -1),
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
enum Stage {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Stage {
    const fn next(self) -> Self {
        match self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}

struct Puzzle {
    infected: Vec<Vector2D<isize>>,
}

impl Puzzle {
    fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
        separated_list1(
            line_ending,
            many1(alt((value(true, char('#')), value(false, char('.'))))),
        )
        .parse_complete(input)
    }

    fn traverse<U>(&self, num_steps: usize, update: U) -> usize
    where
        U: Fn(Stage) -> Stage,
    {
        let mut stages = self
            .infected
            .iter()
            .copied()
            .zip(std::iter::repeat(Stage::Infected))
            .collect::<FxHashMap<_, _>>();
        let mut pos = Vector2D::new(0, 0);
        let mut dir = Direction::default();
        let mut counter = 0;
        for _ in 0..num_steps {
            let (prev, next) = match stages.entry(pos) {
                Entry::Occupied(mut entry) => {
                    let prev = *entry.get();
                    let next = update(prev);
                    if matches!(next, Stage::Clean) {
                        entry.remove();
                    } else {
                        *entry.get_mut() = next;
                    }
                    (prev, next)
                }
                Entry::Vacant(entry) => {
                    let next = update(Stage::Clean);
                    entry.insert(next);
                    (Stage::Clean, next)
                }
            };
            match prev {
                Stage::Clean => {
                    dir = dir.turn_left();
                }
                Stage::Weakened => {}
                Stage::Infected => {
                    dir = dir.turn_right();
                }
                Stage::Flagged => {
                    dir = dir.reverse();
                }
            }
            if matches!(next, Stage::Infected) {
                counter += 1;
            }
            let dp: Vector2D<_> = dir.into();
            pos += dp;
        }
        counter
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let grid = parser::parse_input_str(input, Self::parse_grid)?;
        let nrow = grid.len().cast_signed();
        let ncol = grid[0].len().cast_signed();
        let infected = grid
            .into_iter()
            .enumerate()
            .flat_map(|(ridx, row)| {
                row.into_iter().enumerate().filter_map(move |(cidx, col)| {
                    col.then_some(Vector2D::new(
                        ridx.cast_signed() - nrow / 2,
                        cidx.cast_signed() - ncol / 2,
                    ))
                })
            })
            .collect::<Vec<_>>();
        Ok(Self { infected })
    }

    fn part1(&self) -> String {
        self.traverse(10_000, |s| match s {
            Stage::Clean => Stage::Infected,
            Stage::Infected => Stage::Clean,
            _ => unreachable!("Only two states in part 1"),
        })
        .to_string()
    }

    fn part2(&self) -> String {
        self.traverse(10_000_000, Stage::next).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
