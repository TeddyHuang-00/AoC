use std::{cmp::Reverse, collections::BTreeMap};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::{alpha1, char, line_ending, u16},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Room {
    name: Vec<&'static str>,
    sector: u16,
    checksum: [char; 5],
}

struct Puzzle {
    rooms: Vec<Room>,
}

impl Puzzle {
    fn parse_room(input: &'static str) -> IResult<&'static str, Room> {
        map(
            (
                separated_pair(separated_list1(char('-'), alpha1), char('-'), u16),
                delimited(char('['), take(5usize), char(']')),
            ),
            |((name, sector), checksum): ((Vec<&'static str>, u16), &'static str)| {
                let checksum = *checksum
                    .chars()
                    .collect::<Vec<_>>()
                    .as_array()
                    .unwrap_or_else(|| unreachable!());
                Room {
                    name,
                    sector,
                    checksum,
                }
            },
        )
        .parse_complete(input)
    }

    const fn shift(ch: char, shift: u16) -> char {
        let mut x = (ch as u8) - b'a';
        x = (x + (shift % 26) as u8) % 26 + b'a';
        x as char
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let rooms = parser::parse_input_str(input, separated_list1(line_ending, Self::parse_room))?;
        Ok(Self { rooms })
    }

    fn part1(&self) -> String {
        self.rooms
            .par_iter()
            .filter_map(
                |Room {
                     name,
                     sector,
                     checksum,
                 }| {
                    let mut name = name
                        .iter()
                        .fold(BTreeMap::new(), |mut acc, s| {
                            for ch in s.chars() {
                                acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1u8);
                            }
                            acc
                        })
                        .into_iter()
                        .collect::<Vec<_>>();
                    name.sort_unstable_by_key(|&(ch, cnt)| (Reverse(cnt), ch));

                    name.iter()
                        .zip(checksum.iter())
                        .all(|((c1, _), c2)| c1 == c2)
                        .then_some(u32::from(*sector))
                },
            )
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.rooms
            .par_iter()
            .find_map_any(
                |Room {
                     name,
                     sector,
                     checksum: _,
                 }| {
                    (name[0]
                        .chars()
                        .map(|ch| Self::shift(ch, *sector))
                        .collect::<String>()
                        == "northpole")
                        .then_some(*sector)
                },
            )
            .unwrap_or_else(|| panic!("No solution found in input"))
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
