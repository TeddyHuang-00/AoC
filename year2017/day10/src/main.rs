use anyhow::Result;
use nom::{
    character::complete::{char, u8},
    multi::separated_list1,
};
use util::{Solution, chore, parser};

struct Puzzle {
    lengths: Vec<u8>,
    ascii_lengths: Vec<u8>,
}

impl Puzzle {
    const fn step(pos: u8, offset: u8) -> u8 {
        pos.wrapping_add(offset)
    }

    fn hash(list: &mut [u8], pos: &mut u8, skip: &mut u8, lengths: &[u8]) {
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
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let lengths = parser::parse_input_str(
            input,
            separated_list1(char::<&str, nom::error::Error<&str>>(','), u8),
        )?;
        let mut ascii_lengths = input.trim().chars().map(|ch| ch as u8).collect::<Vec<_>>();
        ascii_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        Ok(Self {
            lengths,
            ascii_lengths,
        })
    }

    fn part1(&self) -> String {
        let mut list = (0..=u8::MAX).collect::<Vec<_>>();
        Self::hash(&mut list, &mut 0, &mut 0, &self.lengths);
        (u16::from(list[0]) * u16::from(list[1])).to_string()
    }

    fn part2(&self) -> String {
        let mut list = (0..=u8::MAX).collect::<Vec<_>>();
        let mut pos = 0;
        let mut skip = 0;
        for _ in 0..64 {
            Self::hash(&mut list, &mut pos, &mut skip, &self.ascii_lengths);
        }
        list.chunks(16)
            .map(|chunk| {
                format!(
                    "{:02x}",
                    chunk
                        .iter()
                        .copied()
                        .reduce(|a, b| a ^ b)
                        .unwrap_or_else(|| unreachable!("Chunk must not be empty"))
                )
            })
            .reduce(|a, b| a + &b)
            .unwrap_or_else(|| unreachable!("List must contain 16 windows"))
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
