use std::fmt::{Display, Write};

use anyhow::Result;
use itertools::Itertools;
use ndarray::{
    Zip,
    parallel::prelude::{IntoParallelIterator, ParallelIterator},
    prelude::*,
};
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Light {
    On,
    Off,
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => f.write_char('#'),
            Self::Off => f.write_char('.'),
        }
    }
}

impl Light {
    fn update(self, neighbors: u8) -> Self {
        match self {
            Self::On if [2, 3].contains(&neighbors) => Self::On,
            Self::Off if neighbors == 3 => Self::On,
            _ => Self::Off,
        }
    }
}

struct Puzzle {
    grid: Array2<Light>,
}

impl Puzzle {
    fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Light>>> {
        separated_list1(
            line_ending,
            many1(alt((
                value(Light::Off, char('.')),
                value(Light::On, char('#')),
            ))),
        )
        .parse_complete(input)
    }

    fn step(grid: &mut Array2<Light>) {
        let neighbors = Array2::from_shape_fn(grid.dim(), |(i, j)| {
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter_map(|(di, dj)| {
                    if di == 0 && dj == 0 {
                        return None;
                    }
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i >= grid.nrows() || j >= grid.ncols() {
                        return None;
                    }
                    Some(match grid[(i, j)] {
                        Light::Off => 0,
                        Light::On => 1,
                    })
                })
                .sum::<u8>()
        });
        Zip::from(grid).and(&neighbors).par_for_each(|g, &n| {
            *g = g.update(n);
        });
    }

    fn always_on(grid: &mut Array2<Light>) {
        [0, grid.nrows() - 1]
            .into_iter()
            .cartesian_product([0, grid.ncols() - 1])
            .for_each(|(i, j)| grid[(i, j)] = Light::On);
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let grid = parser::parse_input_str(input, Self::parse_grid)?;
        let grid = parser::nested_vec_to_array2(grid)?;
        Ok(Self { grid })
    }

    fn part1(&self) -> String {
        let mut grid = self.grid.clone();
        (0..100).for_each(|_| Self::step(&mut grid));
        grid.into_par_iter()
            .filter(|&light| matches!(light, Light::On))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut grid = self.grid.clone();
        Self::always_on(&mut grid);
        (0..100).for_each(|_| {
            Self::step(&mut grid);
            Self::always_on(&mut grid);
        });
        grid.into_par_iter()
            .filter(|&light| matches!(light, Light::On))
            .count()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
