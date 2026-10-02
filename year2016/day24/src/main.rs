use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending, u8},
    combinator::map,
    multi::{many1, many1_count, separated_list1},
};
use rayon::prelude::*;
use util::{Solution, chore, integer_as_key, parser};

#[derive(Clone, Copy)]
enum Cell {
    Wall(u8),
    Empty(u8),
    Number(u8),
}

type Coord = (u8, u8);

#[derive(Clone, Copy)]
struct SearchState {
    current: Coord,
    cost: u16,
    total: u16,
}

integer_as_key!(SearchState, total);

struct Grid {
    walls: Vec<Vec<(u8, u8)>>,
}

impl Grid {
    fn is_wall(&self, coord: Coord) -> bool {
        let (x, y) = coord;
        self.walls.get(y as usize).is_none_or(|row| {
            // Binary search to check if the coordinate is within a wall range
            let mut left = 0;
            let mut right = row.len() - 1;
            if row[left].0 <= x && x <= row[left].1 || row[right].0 <= x && x <= row[right].1 {
                return true;
            }
            while left <= right {
                let mid = usize::midpoint(left, right);
                if row[mid].0 <= x && x <= row[mid].1 {
                    return true;
                }
                if x < row[mid].0 {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            false
        })
    }
}

struct Puzzle {
    dist: BTreeMap<Coord, u16>,
    max_number: u8,
}

impl Puzzle {
    #[allow(clippy::cast_possible_truncation)]
    fn parse_row(input: &str) -> IResult<&str, Vec<Cell>> {
        many1(alt((
            map(many1_count(char('#')), |cnt| Cell::Wall(cnt as u8)),
            map(many1_count(char('.')), |cnt| Cell::Empty(cnt as u8)),
            map(u8, Cell::Number),
        )))
        .parse_complete(input)
    }

    fn brute_force_tsp(&self, return_to_start: bool) -> u16 {
        (1..=self.max_number)
            .permutations(self.max_number as usize)
            .filter_map(|stops| {
                std::iter::once(0)
                    .chain(stops)
                    .chain(if return_to_start { vec![0] } else { vec![] })
                    .array_windows()
                    .try_fold(0, |mut acc, [src, dst]| {
                        self.dist.get(&(src.min(dst), src.max(dst))).map(|d| {
                            acc += d;
                            acc
                        })
                    })
            })
            .min()
            .unwrap_or_else(|| panic!("No solution found"))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let grid = parser::parse_input_str(input, separated_list1(line_ending, Self::parse_row))?;
        let mut walls = Vec::with_capacity(grid.len());
        let mut numbers = BTreeMap::new();
        for (y, row) in grid.iter().enumerate() {
            walls.push(vec![]);
            let mut x = 0;
            for &cell in row {
                match cell {
                    Cell::Wall(len) => {
                        walls[y].push((x, x + len - 1));
                        x += len;
                    }
                    Cell::Empty(len) => x += len,
                    Cell::Number(num) => {
                        #[allow(clippy::cast_possible_truncation)]
                        numbers.insert(num, (x, y as u8));
                        x += 1;
                    }
                }
            }
        }
        let max_number = numbers.keys().copied().max().unwrap_or(0);
        let grid = Grid { walls };
        let dist = numbers
            .iter()
            .cartesian_product(numbers.iter())
            .par_bridge()
            .filter_map(|((&a, &src), (&b, &dst))| {
                if a >= b {
                    return None;
                }
                let heuristic =
                    |coord: Coord| u16::from(coord.0.abs_diff(dst.0) + coord.1.abs_diff(dst.1));
                let mut frontiers = BinaryHeap::from_iter([Reverse(SearchState {
                    current: src,
                    cost: 0,
                    total: heuristic(src),
                })]);
                let mut visited = BTreeSet::from_iter([src]);
                while let Some(Reverse(SearchState {
                    current,
                    cost,
                    total: _,
                })) = frontiers.pop()
                {
                    if current == dst {
                        return Some(((a, b), cost));
                    }
                    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let next = (
                            current.0.wrapping_add_signed(dx),
                            current.1.wrapping_add_signed(dy),
                        );
                        if !grid.is_wall(next) && visited.insert(next) {
                            let cost = cost + 1;
                            frontiers.push(Reverse(SearchState {
                                current: next,
                                cost,
                                total: cost + heuristic(next),
                            }));
                        }
                    }
                }
                None
            })
            .collect();
        Ok(Self { dist, max_number })
    }

    fn part1(&self) -> String {
        self.brute_force_tsp(false).to_string()
    }

    fn part2(&self) -> String {
        self.brute_force_tsp(true).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
