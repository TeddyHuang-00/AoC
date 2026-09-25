use std::collections::HashSet;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u8},
    combinator::value,
    multi::separated_list1,
};
use util::{Solution, chore, parser, vector::Vector2D};

#[derive(Clone, Copy)]
enum Turn {
    Left = -1,
    Right = 1,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    const fn turn(&mut self, turn: Turn) -> Self {
        let idx = (*self as usize + 4).wrapping_add_signed(turn as isize) % 4;
        *self = Self::ALL[idx];
        *self
    }

    const fn to_vector(self) -> Vector2D<isize> {
        let v = self as isize;
        let d = (!v & 2) - 1;
        let (x, y) = if (v & 1) == 1 { (d, 0) } else { (0, d) };
        Vector2D { x, y }
    }
}

type PathSegment = (Vector2D<isize>, Vector2D<isize>);

struct Puzzle {
    instructions: Vec<(Turn, u8)>,
}

impl Puzzle {
    fn parse_instruction(input: &str) -> IResult<&str, (Turn, u8)> {
        (
            alt((value(Turn::Left, char('L')), value(Turn::Right, char('R')))),
            u8,
        )
            .parse_complete(input)
    }

    fn has_intersection(
        path: PathSegment,
        visited: &HashSet<PathSegment>,
    ) -> Option<Vector2D<isize>> {
        let curr = path.1 - path.0;
        let vertical = curr.x == 0;
        visited
            .iter()
            .filter_map(|&other| {
                let prev = other.1 - other.0;
                let dot_product =
                    curr.as_arr()
                        .into_iter()
                        .zip(prev.as_arr())
                        .fold(0, |mut acc, (a, b)| {
                            acc += a * b;
                            acc
                        });
                if dot_product == 0 {
                    let (a, b) = if vertical {
                        (path, other)
                    } else {
                        (other, path)
                    };
                    let ((x, y1, y2), (y, x1, x2)) = ((a.0.x, a.0.y, a.1.y), (b.0.y, b.0.x, b.1.x));
                    (x >= x1.min(x2) && x <= x1.max(x2) && y >= y1.min(y2) && y <= y1.max(y2))
                        .then_some(Vector2D { x, y })
                } else {
                    if (vertical && path.0.x != other.0.x) || (!vertical && path.0.y != other.0.y) {
                        return None;
                    }
                    let (a, b, c, d) = if vertical {
                        (path.0.y, path.1.y, other.0.y, other.1.y)
                    } else {
                        (path.0.x, path.1.x, other.0.x, other.1.x)
                    };
                    let (a, b, c, d) = (a.min(b), a.max(b), c.min(d), c.max(d));
                    if a <= d && b >= c {
                        let (start, end) = (a.max(c), b.min(d));
                        (start..=end)
                            .map(|p| {
                                if vertical {
                                    Vector2D { x: path.0.x, y: p }
                                } else {
                                    Vector2D { x: p, y: path.0.y }
                                }
                            })
                            .min_by_key(|&pos| {
                                let diff = pos - path.0;
                                diff.x.unsigned_abs() + diff.y.unsigned_abs()
                            })
                    } else {
                        None
                    }
                }
            })
            .min_by_key(|&pos| {
                let diff = pos - path.0;
                diff.x.unsigned_abs() + diff.y.unsigned_abs()
            })
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(tag(", "), Self::parse_instruction))?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        let mut position = Vector2D { x: 0, y: 0 };
        let mut direction = Direction::North;
        for &(turn, steps) in &self.instructions {
            direction.turn(turn);
            position += direction.to_vector() * isize::from(steps);
        }
        (position.x.unsigned_abs() + position.y.unsigned_abs()).to_string()
    }

    fn part2(&self) -> String {
        let mut position = Vector2D { x: 0, y: 0 };
        let mut direction = Direction::North;
        let mut visited = HashSet::new();
        for &(turn, steps) in &self.instructions {
            let last = position;
            direction.turn(turn);
            position += direction.to_vector() * isize::from(steps);
            if let Some(intersection) = Self::has_intersection((last, position), &visited) {
                return (intersection.x.unsigned_abs() + intersection.y.unsigned_abs()).to_string();
            }
            visited.insert((last, position - direction.to_vector()));
        }
        panic!("No intersection found");
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
