use anyhow::Result;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

struct Puzzle {
    dims: Vec<[u8; 3]>,
}

impl Puzzle {
    fn parse_dimensions(input: &str) -> IResult<&str, [u8; 3]> {
        use nom::character::complete::u8;
        (u8, tag("x"), u8, tag("x"), u8)
            .parse_complete(input)
            .map(|(rem, (l, _, w, _, h))| (rem, [l, w, h]))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let dims =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_dimensions))?;

        Ok(Self { dims })
    }

    fn part1(&self) -> String {
        self.dims
            .par_iter()
            .copied()
            .map(|[l, w, h]| {
                let [l, w, h] = [u32::from(l), u32::from(w), u32::from(h)];
                let side = [l * w, w * h, l * h];
                2 * (side[0] + side[1] + side[2])
                    + *side
                        .iter()
                        .min()
                        .unwrap_or_else(|| panic!("List should be none empty: {side:?}"))
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.dims
            .par_iter()
            .copied()
            .map(|[l, w, h]| {
                let [l, w, h] = [u32::from(l), u32::from(w), u32::from(h)];
                let longest = l.max(w).max(h);
                2 * (l + w + h - longest) + l * w * h
            })
            .sum::<u32>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
