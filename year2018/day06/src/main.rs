use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i16, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use util::{Solution, chore, parser, vector::Vector2D};

type Point = Vector2D<i16>;

struct Puzzle {
    points: Vec<Point>,
    bbox: Point,
    threshold: u16,
}

impl Puzzle {
    fn parse_coord(input: &str) -> IResult<&str, Point> {
        map(separated_pair(i16, tag(", "), i16), |(x, y)| {
            Point::new(x, y)
        })
        .parse_complete(input)
    }

    fn get_border_set(&self) -> BTreeSet<Point> {
        (0..=self.bbox.y + 1)
            .flat_map(|row| [Point::new(0, row), Point::new(self.bbox.x + 1, row)])
            .chain(
                (0..=self.bbox.x + 1)
                    .flat_map(|col| [Point::new(col, 0), Point::new(col, self.bbox.y + 1)]),
            )
            .filter_map(|border| {
                self.points
                    .iter()
                    .fold((u16::MAX, None), |(mut dist, mut closest), &pt| {
                        let d = pt.x.abs_diff(border.x) + pt.y.abs_diff(border.y);
                        match d {
                            x if x < dist => {
                                dist = x;
                                closest = Some(pt);
                            }
                            x if x == dist => {
                                closest = None;
                            }
                            _ => {}
                        }
                        (dist, closest)
                    })
                    .1
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let mut points =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_coord))?;
        points.sort_unstable();
        let bbox = points
            .iter()
            .copied()
            .reduce(|max, pt| max.op(pt, |(a, b)| a.max(b)))
            .unwrap_or_else(|| panic!("Points must not be empty"));
        let threshold = if E { 32 } else { 10_000 };
        Ok(Self {
            points,
            bbox,
            threshold,
        })
    }

    fn part1(&self) -> String {
        let border = self.get_border_set();
        let areas = (0..=self.bbox.x)
            .into_par_iter()
            .map(|row| {
                (0..=self.bbox.y).fold(BTreeMap::new(), |mut acc, col| {
                    let closest = self
                        .points
                        .iter()
                        .copied()
                        .min_set_by_key(|&pt| pt.x.abs_diff(col) + pt.y.abs_diff(row));
                    if closest.len() == 1 && !border.contains(&closest[0]) {
                        acc.entry(closest[0])
                            .and_modify(|cnt| *cnt += 1)
                            .or_insert(1);
                    }
                    acc
                })
            })
            .reduce(BTreeMap::new, |mut acc, counts| {
                for (pt, c) in counts {
                    acc.entry(pt).and_modify(|cnt| *cnt += c).or_insert(c);
                }
                acc
            });
        areas.into_values().max().unwrap_or(u16::MAX).to_string()
    }

    fn part2(&self) -> String {
        (0..=self.bbox.x)
            .into_par_iter()
            .map(|row| {
                (0..=self.bbox.y).fold(0, |mut acc, col| {
                    let dist_sum = self
                        .points
                        .iter()
                        .map(|&pt| pt.x.abs_diff(col) + pt.y.abs_diff(row))
                        .sum::<u16>();
                    if dist_sum < self.threshold {
                        acc += 1;
                    }
                    acc
                })
            })
            .reduce(|| 0, |acc, cnt| acc + cnt)
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
