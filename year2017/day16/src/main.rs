use std::collections::HashMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{alpha1, char, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char),
}

struct Puzzle {
    moves: Vec<Move>,
}

impl Puzzle {
    fn parse_move(input: &str) -> IResult<&str, Move> {
        alt((
            map(preceded(char('s'), u8), Move::Spin),
            map(
                preceded(char('x'), separated_pair(u8, char('/'), u8)),
                |(x, y)| Move::Exchange(x, y),
            ),
            map(
                preceded(char('p'), separated_pair(alpha1, char('/'), alpha1)),
                |(x, y): (&str, &str)| {
                    Move::Partner(x.as_bytes()[0] as char, y.as_bytes()[0] as char)
                },
            ),
        ))
        .parse_complete(input)
    }

    fn dance(&self, slots: &mut [char; 16]) {
        for &m in &self.moves {
            match m {
                Move::Spin(len) => slots.rotate_right(len.into()),
                Move::Exchange(x, y) => slots.swap(x.into(), y.into()),
                Move::Partner(x, y) => {
                    let x = slots
                        .iter()
                        .copied()
                        .enumerate()
                        .find_map(|(idx, ch)| (ch == x).then_some(idx))
                        .unwrap_or_else(|| panic!("{x} is not in slots"));
                    let y = slots
                        .iter()
                        .copied()
                        .enumerate()
                        .find_map(|(idx, ch)| (ch == y).then_some(idx))
                        .unwrap_or_else(|| panic!("{y} is not in slots"));
                    slots.swap(x, y);
                }
            }
        }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let moves = parser::parse_input_str(input, separated_list1(char(','), Self::parse_move))?;
        Ok(Self { moves })
    }

    fn part1(&self) -> String {
        let mut slots = [' '; 16];
        for (slot, ch) in slots.iter_mut().zip('a'..) {
            *slot = ch;
        }
        self.dance(&mut slots);
        slots.into_iter().collect()
    }

    fn part2(&self) -> String {
        let mut slots = [' '; 16];
        for (slot, ch) in slots.iter_mut().zip('a'..) {
            *slot = ch;
        }
        let rounds = 1_000_000_000;
        let mut seen = HashMap::new();
        let mut idx = 0u32;
        while idx < rounds {
            if let Some(prev) = seen.insert(slots, idx) {
                let cycle_length = idx - prev;
                let remainder = (rounds - idx) % cycle_length;
                slots = seen
                    .into_iter()
                    .find_map(|(state, step)| (step == prev + remainder).then_some(state))
                    .unwrap_or_else(|| {
                        unreachable!("All states within the cycle must have been seen")
                    });
                break;
            }
            self.dance(&mut slots);
            idx += 1;
        }
        slots.into_iter().collect()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
