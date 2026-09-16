use anyhow::Result;
use ndarray::{parallel::prelude::*, prelude::*};
use util::{Solution, chore, parser, vector::Vector2D};

type Coord = Vector2D<usize>;

enum Instruction {
    On(Coord, Coord),
    Off(Coord, Coord),
    Toggle(Coord, Coord),
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_into_coord(s: &str) -> Result<Coord> {
        let Some((x, y)) = s.split_once(',') else {
            anyhow::bail!("Invalid coordinate: {s}");
        };
        Ok(Coord::new(x.parse()?, y.parse()?))
    }

    fn apply_instructions<T>(
        &self,
        init_state: T,
        on: fn(&mut T),
        off: fn(&mut T),
        toggle: fn(&mut T),
    ) -> Array2<T>
    where
        T: Clone + Send,
    {
        let mut grid = Array2::from_elem((1000, 1000), init_state);
        for instruction in &self.instructions {
            let (s, e, update): (Coord, Coord, fn(&mut T)) = match instruction {
                Instruction::On(s, e) => (*s, *e, on),
                Instruction::Off(s, e) => (*s, *e, off),
                Instruction::Toggle(s, e) => (*s, *e, toggle),
            };
            grid.slice_mut(s![s.x..=e.x, s.y..=e.y]).map_inplace(update);
        }

        grid
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions = parser::parse_lines(input.trim(), |line| {
            let parts = line.rsplit(' ').collect::<Vec<_>>();
            let end = Self::parse_into_coord(parts[0])?;
            let start = Self::parse_into_coord(parts[2])?;
            match parts[3] {
                "toggle" => Ok(Instruction::Toggle(start, end)),
                "on" => Ok(Instruction::On(start, end)),
                "off" => Ok(Instruction::Off(start, end)),
                _ => anyhow::bail!("Invalid instruction: {}", parts[3]),
            }
        })?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        let grid = self.apply_instructions(
            false,
            |state| *state = true,
            |state| *state = false,
            |state| *state = !*state,
        );

        grid.into_par_iter()
            .filter(|&&state| state)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let grid = self.apply_instructions(
            0u32,
            |state| *state += 1,
            |state| *state = state.saturating_sub(1),
            |state| *state += 2,
        );

        grid.into_par_iter().sum::<u32>().to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
