use std::mem::swap;

use anyhow::Result;
use util::{Solution, chore, hash::md5};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn as_byte(self) -> u8 {
        match self {
            Self::Up => b'U',
            Self::Down => b'D',
            Self::Left => b'L',
            Self::Right => b'R',
        }
    }

    const fn to_delta(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Clone)]
struct SearchState {
    path: Vec<u8>,
    position: (usize, usize),
}

struct Puzzle {
    passcode: &'static str,
}

impl Puzzle {
    fn bfs_search(&self, shortest: bool) -> Option<Vec<u8>> {
        let mut solution = None;
        let mut frontiers = Vec::from_iter([SearchState {
            path: self.passcode.as_bytes().to_vec(),
            position: (0, 0),
        }]);
        'outer: while !frontiers.is_empty() {
            let mut next_frontiers = Vec::with_capacity(frontiers.len() * 2);
            let (finished, rest): (Vec<_>, Vec<_>) = frontiers
                .iter()
                .partition(|SearchState { path: _, position }| position == &(3, 3));
            for state in finished {
                if shortest {
                    solution = Some(state.path.clone());
                    break 'outer;
                }
                match &solution {
                    Some(path) if path.len() >= state.path.len() => {}
                    _ => solution = Some(state.path.clone()),
                }
            }
            for SearchState { path, position } in rest {
                let hash = md5::digest(path);
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .filter_map(|dir| {
                    let (dx, dy) = dir.to_delta();
                    let (x, y) = (
                        position.0.wrapping_add_signed(dx),
                        position.1.wrapping_add_signed(dy),
                    );
                    if x >= 4 || y >= 4 {
                        return None;
                    }

                    let status = hash[dir as usize / 2];
                    let status = status >> (((1 + dir as usize) % 2) * 4);
                    let status = status & 0xf;
                    match status {
                        0..11 => None,
                        11..16 => Some((dir, (x, y))),
                        _ => unreachable!("Invalid status: {status}"),
                    }
                })
                .for_each(|(dir, position)| {
                    let mut path = path.clone();
                    path.push(dir.as_byte());
                    next_frontiers.push(SearchState { path, position });
                });
            }

            swap(&mut frontiers, &mut next_frontiers);
        }

        solution
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let passcode = input.trim();
        Ok(Self { passcode })
    }

    fn part1(&self) -> String {
        self.bfs_search(true).map_or_else(
            || panic!("No solution found"),
            |path| {
                let path = String::from_utf8_lossy(&path[self.passcode.len()..]);
                path.to_string()
            },
        )
    }

    fn part2(&self) -> String {
        self.bfs_search(false).map_or_else(
            || panic!("No solution found"),
            |path| path[self.passcode.len()..].len().to_string(),
        )
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
