use anyhow::Result;
use ndarray::{parallel::prelude::*, prelude::*};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, usize},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Instruction {
    Rect { ncol: usize, nrow: usize },
    Column { col: usize, shift: usize },
    Row { row: usize, shift: usize },
}

struct Puzzle {
    screen: Array2<bool>,
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                preceded(tag("rect "), separated_pair(usize, char('x'), usize)),
                |(ncol, nrow)| Instruction::Rect { ncol, nrow },
            ),
            map(
                preceded(
                    tag("rotate column x="),
                    separated_pair(usize, tag(" by "), usize),
                ),
                |(col, shift)| Instruction::Column { col, shift },
            ),
            map(
                preceded(
                    tag("rotate row y="),
                    separated_pair(usize, tag(" by "), usize),
                ),
                |(row, shift)| Instruction::Row { row, shift },
            ),
        ))
        .parse_complete(input)
    }

    fn execute(&self) -> Array2<bool> {
        let mut screen = self.screen.clone();
        for &instruction in &self.instructions {
            match instruction {
                Instruction::Rect { ncol, nrow } => {
                    screen
                        .slice_mut(s![..nrow, ..ncol])
                        .map_inplace(|light| *light = true);
                }
                Instruction::Column { col, shift } => {
                    let origin = screen.column(col).to_owned();
                    let nrows = origin.len();
                    let shift = (shift % nrows).cast_signed();
                    if shift == 0 {
                        continue;
                    }
                    let mut column = screen.column_mut(col);
                    column
                        .slice_mut(s![..shift])
                        .assign(&origin.slice(s![-shift..]));
                    column
                        .slice_mut(s![shift..])
                        .assign(&origin.slice(s![..-shift]));
                }
                Instruction::Row { row, shift } => {
                    let origin = screen.row(row).to_owned();
                    let ncols = origin.len();
                    let shift = (shift % ncols).cast_signed();
                    if shift == 0 {
                        continue;
                    }
                    let mut row = screen.row_mut(row);
                    row.slice_mut(s![..shift])
                        .assign(&origin.slice(s![-shift..]));
                    row.slice_mut(s![shift..])
                        .assign(&origin.slice(s![..-shift]));
                }
            }
        }
        screen
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let screen = if E {
            Array2::from_elem((3, 7), false)
        } else {
            Array2::from_elem((6, 50), false)
        };
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        Ok(Self {
            screen,
            instructions,
        })
    }

    fn part1(&self) -> String {
        let screen = self.execute();
        screen
            .into_par_iter()
            .filter(|&&light| light)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let screen = self.execute();
        if self.screen.ncols() == 50 {
            // Yes, we sometimes DO need human wisdom... I mean, vision.
            "\n".to_string()
                + &screen
                    .outer_iter()
                    .into_par_iter()
                    .map(|line| {
                        line.into_iter()
                            .map(|&light| if light { '#' } else { '.' })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
        } else {
            String::default()
        }
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
