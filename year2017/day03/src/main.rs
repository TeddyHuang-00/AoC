use std::collections::BTreeMap;

use anyhow::Result;
use itertools::Itertools;
use util::{Solution, chore};

#[derive(Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

struct Puzzle {
    data: usize,
}

impl Puzzle {
    const fn idx_to_coord(idx: usize) -> Coord {
        let k = {
            let mut k = 1;
            while (2 * k + 1) * (2 * k + 1) < idx {
                k += 1;
            }
            k
        };
        let side_len = 2 * k;
        let inner = (side_len - 1) * (side_len - 1);
        let offset = idx - inner;
        let side = (offset - 1) / side_len;
        let (start, step) = {
            let k = k.cast_signed();
            match side {
                0 => ((k, -k), (0, 1)),
                1 => ((k, k), (-1, 0)),
                2 => ((-k, k), (0, -1)),
                3 => ((-k, -k), (1, 0)),
                _ => unreachable!(),
            }
        };
        let num_steps = (match offset % side_len {
            0 => side_len,
            x => x,
        })
        .cast_signed();
        let x = start.0 + step.0 * num_steps;
        let y = start.1 + step.1 * num_steps;
        Coord { x, y }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let data = input.trim().parse()?;
        Ok(Self { data })
    }

    fn part1(&self) -> String {
        let coord = Self::idx_to_coord(self.data);
        (coord.x.abs_diff(0) + coord.y.abs_diff(0)).to_string()
    }

    fn part2(&self) -> String {
        let mut seen = BTreeMap::from_iter([((0, 0), 1)]);
        for idx in 2..=self.data {
            let coord = Self::idx_to_coord(idx);
            let sum = (-1..=1)
                .cartesian_product(-1..=1)
                .filter_map(|(dx, dy)| {
                    if dx == 0 && dy == 0 {
                        return None;
                    }
                    let x = coord.x + dx;
                    let y = coord.y + dy;
                    seen.get(&(x, y))
                })
                .sum::<usize>();
            if sum > self.data {
                return sum.to_string();
            }
            seen.insert((coord.x, coord.y), sum);
        }
        panic!("No solution found")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
