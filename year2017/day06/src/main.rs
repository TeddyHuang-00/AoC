use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
};

use anyhow::Result;
use nom::{
    character::complete::{space1, u8},
    multi::separated_list1,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Banks {
    state: u64,
    count: u8,
}

impl From<Vec<u8>> for Banks {
    fn from(value: Vec<u8>) -> Self {
        // At most 16 banks
        assert!(value.len() <= 16);
        // No bank has more than 15 blocks
        assert!(value.iter().copied().max().is_some_and(|max| max < 16));
        // At most one with 15 blocks
        assert!(value.iter().copied().filter(|&b| b >= 15).count() <= 1);
        #[allow(clippy::cast_possible_truncation, reason = "16 is within u8")]
        let count = value.len() as u8;
        let state = value.into_iter().enumerate().fold(0, |mut acc, (i, b)| {
            acc |= u64::from(b) << (i * 4);
            acc
        });
        Self { state, count }
    }
}

impl Banks {
    fn redistribute(&mut self) {
        let (idx, max) = (0..self.count)
            .map(|idx| (idx, (self.state >> (idx * 4)) & 0xf))
            .max_by_key(|&(idx, bank)| (bank, Reverse(idx)))
            .unwrap_or_else(|| panic!("Count must not be 0"));
        self.state &= !(0xf << (idx * 4));
        let equal_part = max / u64::from(self.count);
        #[allow(
            clippy::cast_possible_truncation,
            reason = "We have assured max is never larger than 16"
        )]
        let distribution = (0..self.count).fold(0, |mut acc, i| {
            acc |= (equal_part + u64::from(i > 0 && i <= max as u8 % self.count))
                << (((idx + i) % self.count) * 4);
            acc
        });
        self.state += distribution;
    }
}

struct Puzzle {
    banks: Banks,
}

impl Puzzle {}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let banks = parser::parse_input_str(
            input,
            separated_list1(space1::<&str, nom::error::Error<&str>>, u8),
        )?;
        let banks = Banks::from(banks);
        Ok(Self { banks })
    }

    fn part1(&self) -> String {
        let mut banks = self.banks;
        let mut seen = BTreeSet::default();
        let mut count = 0;
        while !seen.contains(&banks.state) {
            seen.insert(banks.state);
            banks.redistribute();
            count += 1;
        }
        count.to_string()
    }

    fn part2(&self) -> String {
        let mut banks = self.banks;
        let mut seen = BTreeMap::default();
        let mut count = 0;
        while !seen.contains_key(&banks.state) {
            seen.insert(banks.state, count);
            banks.redistribute();
            count += 1;
        }
        (count - seen[&banks.state]).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
