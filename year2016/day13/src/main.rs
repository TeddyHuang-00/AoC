use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, VecDeque},
};

use anyhow::Result;
use util::{Solution, chore, integer_as_key};

type Coord = (u32, u32);

#[derive(Clone, Copy)]
struct SearchState {
    coord: Coord,
    steps: u32,
    total: u32,
}

integer_as_key!(SearchState, total);

impl SearchState {
    const fn new(coord: Coord, steps: u32, heuristic: u32) -> Self {
        Self {
            coord,
            steps,
            total: steps + heuristic,
        }
    }
}

struct Puzzle {
    salt: u32,
    target: (u32, u32),
}

impl Puzzle {
    const fn is_wall(&self, x: u32, y: u32) -> bool {
        (x * x + 3 * x + 2 * x * y + y + y * y + self.salt).count_ones() % 2 == 1
    }

    const fn heuristic(&self, coord: Coord) -> u32 {
        self.target.0.abs_diff(coord.0) + self.target.1.abs_diff(coord.1)
    }

    fn search<I, G, F, T, O, Q>(&self, init: I, get: G, callback: F, terminated: T) -> O
    where
        I: FnOnce() -> Q,
        G: Fn(&mut Q) -> Option<SearchState>,
        F: Fn(Coord, u32, &mut Q),
        T: Fn(Coord, u32, &BTreeSet<Coord>) -> Option<O>,
    {
        let mut visited = BTreeSet::new();
        let mut frontiers = init();
        while let Some(SearchState {
            coord: (x, y),
            steps,
            total: _,
        }) = get(&mut frontiers)
        {
            if let Some(result) = terminated((x, y), steps, &visited) {
                return result;
            }
            // Defer the insertion into visited until expanded to make the
            // counting accurate
            if !visited.insert((x, y)) {
                continue;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
                if !self.is_wall(x, y) {
                    callback((x, y), steps, &mut frontiers);
                }
            }
        }

        panic!("No solution found")
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let salt = input.trim().parse()?;
        let target = if E { (7, 4) } else { (31, 39) };
        Ok(Self { salt, target })
    }

    fn part1(&self) -> String {
        let start = (1, 1);
        self.search(
            || BinaryHeap::from_iter([Reverse(SearchState::new(start, 0, self.heuristic(start)))]),
            |frontiers| frontiers.pop().map(|Reverse(state)| state),
            |coord, steps, frontiers| {
                frontiers.push(Reverse(SearchState::new(
                    coord,
                    steps + 1,
                    self.heuristic(coord),
                )));
            },
            |coord, steps, _| (coord == self.target).then_some(steps),
        )
        .to_string()
    }

    fn part2(&self) -> String {
        let start = (1, 1);
        self.search(
            || VecDeque::from_iter([SearchState::new(start, 0, 0)]),
            std::collections::VecDeque::pop_front,
            |coord, steps, frontiers| frontiers.push_back(SearchState::new(coord, steps + 1, 0)),
            |_, steps, visited| (steps > 50).then_some(visited.len()),
        )
        .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
