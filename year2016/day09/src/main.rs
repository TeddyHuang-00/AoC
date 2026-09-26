use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{take, take_till1},
    character::complete::{char, usize},
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{delimited, separated_pair},
};
use util::{Solution, chore, parser};

enum Partial {
    Slice(&'static str),
    Compress(usize, usize),
}

#[derive(Clone, Copy)]
enum SegmentV1 {
    Normal(&'static str),
    Compressed(&'static str, usize),
}

impl SegmentV1 {
    const fn len(self) -> usize {
        match self {
            Self::Normal(s) => s.len(),
            Self::Compressed(s, cnt) => s.len() * cnt,
        }
    }
}

enum SegmentV2 {
    Normal(&'static str),
    Compressed(Vec<Self>, usize),
}

impl SegmentV2 {
    fn len(&self) -> usize {
        match self {
            Self::Normal(s) => s.len(),
            Self::Compressed(segs, cnt) => segs.iter().map(Self::len).sum::<usize>() * cnt,
        }
    }
}

struct Puzzle {
    v1: Vec<SegmentV1>,
    v2: Vec<SegmentV2>,
}

impl Puzzle {
    fn parse_segment_v1(input: &'static str) -> IResult<&'static str, SegmentV1> {
        let (input, partial) = alt((
            map(
                delimited(
                    char('('),
                    separated_pair(usize, char('x'), usize),
                    char(')'),
                ),
                |(len, cnt)| Partial::Compress(len, cnt),
            ),
            map(take_till1(|ch: char| ch == '(' || ch == '\n'), |slice| {
                Partial::Slice(slice)
            }),
        ))
        .parse_complete(input)?;
        match partial {
            Partial::Slice(s) => Ok((input, SegmentV1::Normal(s))),
            Partial::Compress(len, cnt) => {
                map(take(len), |slice| SegmentV1::Compressed(slice, cnt)).parse_complete(input)
            }
        }
    }

    fn parse_segment_v2(input: &'static str) -> IResult<&'static str, SegmentV2> {
        let (input, partial) = Self::parse_segment_v1(input)?;
        match partial {
            SegmentV1::Normal(s) => Ok((input, SegmentV2::Normal(s))),
            SegmentV1::Compressed(inner, cnt) => {
                let (_, segment) = map(all_consuming(many1(Self::parse_segment_v2)), |segs| {
                    SegmentV2::Compressed(segs, cnt)
                })
                .parse_complete(inner)?;
                Ok((input, segment))
            }
        }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let v1 = parser::parse_input_str(input, many1(Self::parse_segment_v1))?;
        let v2 = parser::parse_input_str(input, many1(Self::parse_segment_v2))?;
        Ok(Self { v1, v2 })
    }

    fn part1(&self) -> String {
        self.v1
            .iter()
            .map(|segment| segment.len())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.v2
            .iter()
            .map(SegmentV2::len)
            .sum::<usize>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
