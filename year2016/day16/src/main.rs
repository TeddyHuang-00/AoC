use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::{many1, many1_count},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Segment(i8);

impl Segment {
    const ZERO: Self = Self(1);

    const fn len(self) -> u8 {
        self.0.unsigned_abs()
    }

    const fn flip(self) -> Self {
        Self(-self.0)
    }

    const fn checksum(self, carry: &mut bool) -> Self {
        let count = self.0.abs() - if *carry { 1 } else { 0 };
        *carry = count % 2 == 1;
        Self(-count / 2)
    }
}

#[derive(Clone)]
struct Sequence {
    segments: Vec<Segment>,
    length: usize,
}

impl Sequence {
    fn expand(mut self) -> Self {
        let n = self.segments.len();
        self.segments.reserve(n + 1);
        let last = self.segments[n - 1];

        for i in (0..n).rev() {
            let seg = self.segments[i].flip();
            self.segments.push(seg);
        }

        // Add the middle zero to either left or right
        if last.0 > 0 {
            self.segments[n - 1].0 += 1;
        } else {
            self.segments[n].0 += 1;
        }

        self.length = self.length * 2 + 1;
        self
    }

    fn truncate(&mut self, length: usize) {
        let mut cum = 0;
        for (idx, segment) in self.segments.iter_mut().enumerate() {
            cum += segment.0.unsigned_abs() as usize;
            #[allow(clippy::cast_possible_truncation)]
            if cum >= length {
                *segment = Segment(
                    segment.0.signum()
                        * segment
                            .0
                            .abs()
                            .min((segment.len() as usize + length - cum) as i8),
                );
                self.segments.truncate(idx + 1);
                self.length = length;
                return;
            }
        }
    }

    fn checksum(self) -> Self {
        let mut segments: Vec<Segment> = Vec::with_capacity(self.length / 2);
        let mut carry = false;

        for segment in self.segments {
            if carry {
                match segments.last_mut() {
                    Some(last) if last.0 > 0 => last.0 += 1,
                    _ => segments.push(Segment::ZERO),
                }
            }

            let checksum = segment.checksum(&mut carry);
            if checksum.0 != 0 {
                match segments.last_mut() {
                    Some(last) if last.0 < 0 => last.0 += checksum.0,
                    _ => segments.push(checksum),
                }
            }
        }

        Self {
            segments,
            length: self.length / 2,
        }
    }
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &seg in &self.segments {
            match seg.0.signum() {
                -1 => f.write_str(&"1".repeat(seg.len() as usize))?,
                1 => f.write_str(&"0".repeat(seg.len() as usize))?,
                _ => panic!("Segment should be either 1 or 0"),
            }
        }
        Ok(())
    }
}

struct Puzzle {
    seed: Sequence,
    target_length: (usize, usize),
}

impl Puzzle {
    #[allow(clippy::cast_possible_truncation)]
    fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
        map(
            many1(alt((
                map(many1_count(char('1')), |count| Segment(-(count as i8))),
                map(many1_count(char('0')), |count| Segment(count as i8)),
            ))),
            |segments| {
                let length = segments.iter().map(|s| usize::from(s.len())).sum();
                Sequence { segments, length }
            },
        )
        .parse_complete(input)
    }

    fn fill(&self, length: usize) -> Sequence {
        let mut seq = self.seed.clone();
        while seq.length < length {
            seq = seq.expand();
        }
        seq.truncate(length);
        while seq.length.is_multiple_of(2) {
            seq = seq.checksum();
        }
        seq
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let seed = parser::parse_input_str(input, Self::parse_sequence)?;
        let target_length = if E { (20, 20) } else { (272, 35_651_584) };
        Ok(Self {
            seed,
            target_length,
        })
    }

    fn part1(&self) -> String {
        let seq = self.fill(self.target_length.0);
        format!("{seq}")
    }

    fn part2(&self) -> String {
        let seq = self.fill(self.target_length.1);
        format!("{seq}")
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
