use anyhow::Result;
use ndarray::prelude::*;
use util::{Solution, chore};

struct Puzzle {
    grid: Array2<i8>,
}

impl Puzzle {
    #[inline]
    const fn cell_power_level(sn: isize, x: isize, y: isize) -> i8 {
        let mut out = x + 10;
        out *= y;
        out += sn;
        out *= x + 10;
        out /= 100;
        #[allow(clippy::cast_possible_truncation, reason = "Single digit cast into i8")]
        let out = (out % 10) as i8;
        out - 5
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let serial_number = input.trim().parse()?;
        let grid = Array2::from_shape_fn((300, 300), |(x, y)| {
            let x = (x + 1).cast_signed();
            let y = (y + 1).cast_signed();
            Self::cell_power_level(serial_number, x, y)
        });
        Ok(Self { grid })
    }

    fn part1(&self) -> String {
        let (idx, _) = self
            .grid
            .windows((3, 3))
            .into_iter()
            .enumerate()
            .map(|(idx, win)| (idx, win.sum()))
            .max_by_key(|&(_, sum)| sum)
            .unwrap_or_else(|| unreachable!("Grid is not empty"));
        let stride = 300 - 2;
        let x = idx / stride + 1;
        let y = idx % stride + 1;
        format!("{x},{y}")
    }

    fn part2(&self) -> String {
        let unit = self.grid.mapv(i32::from);
        let mut prev1 = unit.clone();
        let mut prev2 = Array2::zeros((1, 1));
        let (mut x, mut y, mut s, mut max) = (0, 0, 0, i32::MIN);
        for size in 2..=300isize {
            // Divide larger squares into 4 smaller ones
            let mut next = &prev1.slice(s![..-1, ..-1])
                + &prev1.slice(s![1.., 1..])
                + unit.slice(s![size - 1.., ..1 - size])
                + unit.slice(s![..1 - size, size - 1..]);
            #[allow(clippy::reversed_empty_ranges, reason = "Syntax for ndarray slices")]
            if size > 2 {
                // Subtract the central part which is counted twice
                next -= &prev2.slice(s![1..-1, 1..-1]);
            }
            let (idx, val_max) = next
                .iter()
                .copied()
                .enumerate()
                .max_by_key(|&(_, val)| val)
                .unwrap_or_else(|| unreachable!("Grid must not be empty"));
            if val_max > max {
                let stride = 300 - size.cast_unsigned() + 1;
                x = idx / stride + 1;
                y = idx % stride + 1;
                s = size;
                max = val_max;
            }
            (prev1, prev2) = (next, prev1);
        }
        format!("{x},{y},{s}")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
