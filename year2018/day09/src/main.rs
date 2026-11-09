use std::collections::VecDeque;

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{u32, usize},
    sequence::{separated_pair, terminated},
};
use util::{Solution, chore, parser};

struct Puzzle {
    num_players: usize,
    last_marble: u32,
}

impl Puzzle {
    fn parse_setting(input: &str) -> IResult<&str, (usize, u32)> {
        separated_pair(
            terminated(usize, tag(" players")),
            tag("; last marble is worth "),
            terminated(u32, tag(" points")),
        )
        .parse_complete(input)
    }

    fn play(&self, mult: u32) -> u32 {
        let mut scores = vec![0; self.num_players];
        let mut marbles = VecDeque::with_capacity(self.last_marble as usize * mult as usize);
        marbles.push_back(0);
        for (marble, player) in (1..=self.last_marble * mult).zip((0..self.num_players).cycle()) {
            if marble % 23 == 0 {
                marbles.rotate_right(7);
                scores[player] += marble + marbles.pop_back().unwrap_or_else(|| unreachable!());
                marbles.rotate_left(1);
            } else {
                marbles.rotate_left(1);
                marbles.push_back(marble);
            }
        }
        scores.into_iter().max().unwrap_or_else(|| unreachable!())
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (num_players, last_marble) = parser::parse_input_str(input, Self::parse_setting)?;
        Ok(Self {
            num_players,
            last_marble,
        })
    }

    fn part1(&self) -> String {
        self.play(1).to_string()
    }

    fn part2(&self) -> String {
        self.play(100).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
