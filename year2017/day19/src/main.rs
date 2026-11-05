use anyhow::Result;
use ndarray::prelude::*;
use util::{Solution, chore, vector::Vector2D};

#[derive(Clone, Copy)]
enum Waypoint {
    Turning(Vector2D<usize>),
    Checkpoint(Vector2D<usize>, char),
}

impl Waypoint {
    const fn coord(self) -> Vector2D<usize> {
        match self {
            Self::Checkpoint(coord, _) | Self::Turning(coord) => coord,
        }
    }
}

struct Puzzle {
    waypoints: Vec<Waypoint>,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let nrow = input.lines().count();
        let ncol = input
            .lines()
            .map(str::len)
            .max()
            .unwrap_or_else(|| panic!("Expect map to be non-empty"));
        let map = Array2::from_shape_vec(
            (nrow, ncol),
            input
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .chain(std::iter::repeat_n(' ', ncol - line.len()))
                })
                .collect(),
        )?;
        let mut waypoints = vec![];
        let mut pos = Vector2D::new(
            0,
            map.slice(s![0, ..])
                .iter()
                .enumerate()
                .find_map(|(col, &ch)| (ch == '|').then_some(col))
                .unwrap_or_else(|| panic!("No starting point found at top")),
        );
        let mut direction = Vector2D::new(1, 0);
        waypoints.push(Waypoint::Turning(pos));
        while map[pos.as_arr()] != ' ' {
            match map[pos.as_arr()] {
                '+' => {
                    waypoints.push(Waypoint::Turning(pos));
                    let trans = Vector2D::new(direction.y, direction.x);
                    let next_step = pos.op(trans, |(p, dp)| p.wrapping_add_signed(dp));
                    if let Some(&ch) = map.get(next_step.as_arr())
                        && ch != ' '
                    {
                        direction = trans;
                    } else {
                        direction = -trans;
                    }
                }
                ch if ch.is_alphabetic() => {
                    waypoints.push(Waypoint::Checkpoint(pos, ch));
                }
                _ => {}
            }
            pos = pos.op(direction, |(p, dp)| p.wrapping_add_signed(dp));
        }

        Ok(Self { waypoints })
    }

    fn part1(&self) -> String {
        self.waypoints
            .iter()
            .filter_map(|wp| match wp {
                Waypoint::Checkpoint(_, ch) => Some(ch),
                Waypoint::Turning(_) => None,
            })
            .collect()
    }

    fn part2(&self) -> String {
        // Account for the very first step
        (1 + self
            .waypoints
            .array_windows()
            .map(|[start, end]| {
                let start = start.coord();
                let end = end.coord();
                let seg = start.op(end, |(s, e)| s.abs_diff(e));
                seg.x + seg.y
            })
            .sum::<usize>())
        .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
