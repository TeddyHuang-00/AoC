use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till1},
    character::complete::{char, line_ending, space1, u8, u16},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use util::{Solution, chore, integer_as_key, parser};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy)]
struct Node {
    size: u16,
    used: u16,
    avail: u16,
}

#[derive(Clone, Copy)]
struct SearchState {
    empty: Coord,
    cost: u16,
    total: u16,
}

integer_as_key!(SearchState, total);

impl SearchState {
    const fn heuristic(current: Coord, target: Coord) -> u8 {
        current.x.abs_diff(target.x) + current.y.abs_diff(target.y)
    }

    const fn new(current: Coord, target: Coord, cost: u16) -> Self {
        let total = cost + Self::heuristic(current, target) as u16;
        Self {
            empty: current,
            cost,
            total,
        }
    }
}

struct Puzzle {
    nodes: BTreeMap<Coord, Node>,
}

impl Puzzle {
    fn parse_header(input: &str) -> IResult<&str, ()> {
        value(
            (),
            (
                take_till1(|ch| ch == '\n'),
                line_ending,
                take_till1(|ch| ch == '\n'),
                line_ending,
            ),
        )
        .parse_complete(input)
    }

    fn parse_node(input: &str) -> IResult<&str, (Coord, Node)> {
        (
            map(
                preceded(
                    tag("/dev/grid/node"),
                    (preceded(tag("-x"), u8), preceded(tag("-y"), u8)),
                ),
                |(x, y)| Coord { x, y },
            ),
            map(
                (
                    delimited(space1, u16, char('T')),
                    delimited(space1, u16, char('T')),
                    delimited(space1, u16, char('T')),
                    delimited(space1, u8, char('%')),
                ),
                |(size, used, avail, _)| Node { size, used, avail },
            ),
        )
            .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let nodes = parser::parse_input_str(
            input,
            preceded(
                Self::parse_header,
                separated_list1(line_ending, Self::parse_node),
            ),
        )?
        .into_iter()
        .collect();
        Ok(Self { nodes })
    }

    fn part1(&self) -> String {
        self.nodes
            .iter()
            .cartesian_product(self.nodes.iter())
            .filter(|((ca, na), (cb, nb))| ca != cb && na.used > 0 && na.used <= nb.avail)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let (empty, capacity) = self
            .nodes
            .iter()
            .find_map(|(&coord, &node)| (node.used == 0).then_some((coord, node.size)))
            .unwrap_or_else(|| panic!("No empty node found in grid"));
        let (mut walls, (max_x, max_y)) = self.nodes.iter().fold(
            (BTreeSet::new(), (0, 0)),
            |(mut walls, (mut max_x, mut max_y)), (&coord, &node)| {
                if coord.x > max_x {
                    max_x = coord.x;
                }
                if coord.y > max_y {
                    max_y = coord.y;
                }
                if node.used > capacity {
                    walls.insert(coord);
                }
                (walls, (max_x, max_y))
            },
        );
        let target = Coord { x: max_x, y: 0 };
        assert!(
            walls.iter().all(|coord| coord.y > 1),
            "Expect no walls in the first two rows, otherwise the quick solution will not work"
        );

        // distance for empty moving to target's next step, as there might be
        // walls in the way, we use A* search
        let empty_to_position = {
            let destination = Coord {
                // Move empty to target's next step
                x: target.x - 1,
                ..target
            };
            let mut frontiers =
                BinaryHeap::from_iter([Reverse(SearchState::new(empty, destination, 0))]);
            let mut visited = BTreeSet::from_iter([empty]);
            // We don't want the empty to pass through the target while moving
            // into position
            walls.insert(target);
            let mut dist = 0;

            while let Some(Reverse(SearchState {
                empty,
                cost,
                total: _,
            })) = frontiers.pop()
            {
                if empty == destination {
                    dist = cost;
                    break;
                }
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let next = Coord {
                        x: empty.x.wrapping_add_signed(dx),
                        y: empty.y.wrapping_add_signed(dy),
                    };
                    if next.x > max_x
                        || next.y > max_y
                        || walls.contains(&next)
                        || !visited.insert(next)
                    {
                        continue;
                    }
                    frontiers.push(Reverse(SearchState::new(next, destination, cost + 1)));
                }
            }

            dist
        };

        // The rest of the path is a simple calculation, since no walls are in
        // the way, we just repeat the pattern of moving empty to target's next
        // step and swapping empty and target.

        // distance for target moving to the destination
        let target_to_dest = u16::from(target.x);

        // distance for the empty moving to the target's next step for the rest
        // of the path
        let empty_rest_moves = 4 * u16::from(target.x - 1);

        (empty_to_position + target_to_dest + empty_rest_moves).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
