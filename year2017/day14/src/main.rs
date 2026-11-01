use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    grid: [u128; 128],
}

impl Puzzle {
    const fn step(pos: u8, offset: u8) -> u8 {
        pos.wrapping_add(offset)
    }

    fn single_pass(list: &mut [u8], pos: &mut u8, skip: &mut u8, lengths: &[u8]) {
        for &len in lengths {
            for offset in 0..len / 2 {
                list.swap(
                    usize::from(Self::step(*pos, offset)),
                    usize::from(Self::step(*pos, len - 1 - offset)),
                );
            }
            *pos = Self::step(*pos, len);
            *pos = Self::step(*pos, *skip);
            *skip = skip.wrapping_add(1);
        }
    }

    fn hash(message: &[u8]) -> u128 {
        let mut message = message.to_vec();
        message.extend_from_slice(&[17, 31, 73, 47, 23]);

        let mut list = (0..=u8::MAX).collect::<Vec<_>>();
        let mut pos = 0;
        let mut skip = 0;
        for _ in 0..64 {
            Self::single_pass(&mut list, &mut pos, &mut skip, &message);
        }
        list.chunks(16).fold(0u128, |acc, chunk| {
            let res = chunk
                .iter()
                .copied()
                .reduce(|a, b| a ^ b)
                .unwrap_or_else(|| unreachable!("Chunk must not be empty"));
            acc << 8 | u128::from(res)
        })
    }

    fn occupied(grid: &[u128; 128], x: u8, y: u8) -> bool {
        grid[usize::from(y)] >> x & 1 == 1
    }

    fn set(grid: &mut [u128; 128], x: u8, y: u8) {
        grid[usize::from(y)] |= 1 << x;
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let seed = input.trim().to_string() + "-";
        let mut grid = [0; 128];
        for (idx, row) in grid.iter_mut().enumerate() {
            let message = seed.clone() + &idx.to_string();
            *row = Self::hash(message.as_bytes());
        }
        Ok(Self { grid })
    }

    fn part1(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.count_ones())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut visited = [0; 128];
        let mut count = 0;
        for x in 0..128 {
            for y in 0..128 {
                if Self::occupied(&self.grid, x, y) && !Self::occupied(&visited, x, y) {
                    count += 1;
                    let mut frontiers = vec![(x, y)];
                    while let Some((x, y)) = frontiers.pop() {
                        if Self::occupied(&visited, x, y) {
                            continue;
                        }
                        Self::set(&mut visited, x, y);
                        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let x = x.wrapping_add_signed(dx);
                            let y = y.wrapping_add_signed(dy);
                            if x < 128
                                && y < 128
                                && Self::occupied(&self.grid, x, y)
                                && !Self::occupied(&visited, x, y)
                            {
                                frontiers.push((x, y));
                            }
                        }
                    }
                }
            }
        }
        count.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
