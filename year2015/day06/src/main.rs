use anyhow::Result;
use ndarray::{parallel::prelude::*, prelude::*};
use nom::{
    IResult, Parser, branch::alt, bytes::tag, character::complete::line_ending, combinator::value,
    multi::separated_list1, sequence::separated_pair,
};
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
    fn parse_coord(input: &str) -> IResult<&str, Coord> {
        use nom::character::{char, complete::usize};
        separated_pair(usize, char(','), usize)
            .parse_complete(input)
            .map(|(rem, (x, y))| (rem, Coord::new(x, y)))
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            (
                value(1, tag("turn on ")),
                separated_pair(Self::parse_coord, tag(" through "), Self::parse_coord),
            ),
            (
                value(-1, tag("turn off ")),
                separated_pair(Self::parse_coord, tag(" through "), Self::parse_coord),
            ),
            (
                value(0, tag("toggle ")),
                separated_pair(Self::parse_coord, tag(" through "), Self::parse_coord),
            ),
        ))
        .parse_complete(input)
        .map(|(rem, (op, (s, e)))| {
            (
                rem,
                match op {
                    -1 => Instruction::Off(s, e),
                    0 => Instruction::Toggle(s, e),
                    1 => Instruction::On(s, e),
                    _ => unreachable!("Only three possible states"),
                },
            )
        })
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
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
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
