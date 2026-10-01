use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, usize},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnCharPosition(char),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

impl Instruction {
    fn apply(self, input: &mut Vec<char>) {
        match self {
            Self::SwapPosition(x, y) => {
                input.swap(x, y);
            }
            Self::SwapLetter(x, y) => {
                for l in input.iter_mut() {
                    match l {
                        l if *l == x => *l = y,
                        l if *l == y => *l = x,
                        _ => {}
                    }
                }
            }
            Self::RotateLeft(step) => {
                input.rotate_left(step);
            }
            Self::RotateRight(step) => {
                input.rotate_right(step);
            }
            Self::RotateBasedOnCharPosition(ch) => {
                let len = input.len();
                let pos = input
                    .iter()
                    .position(|c| *c == ch)
                    .unwrap_or_else(|| panic!("{ch} not found in {input:?}"));
                input.rotate_right((pos + 1 + usize::from(pos >= 4)) % len);
            }
            Self::ReversePositions(x, y) => {
                input[x..=y].reverse();
            }
            Self::MovePosition(x, y) => {
                let c = input.remove(x);
                input.insert(y, c);
            }
        }
    }

    fn reverse(self, input: &mut Vec<char>) {
        match self {
            Self::SwapPosition(_, _) | Self::SwapLetter(_, _) | Self::ReversePositions(_, _) => {
                self.apply(input);
            }
            Self::RotateLeft(step) => {
                input.rotate_right(step);
            }
            Self::RotateRight(step) => {
                input.rotate_left(step);
            }
            Self::RotateBasedOnCharPosition(ch) => {
                // Note that this reversed rotation logic is only valid for
                // certain input length, which includes the actual input length
                // of 8. Other values, such as 5 used in the example, will not
                // work as it will not be a one-to-one mapping.
                let len = input.len();
                let pos = input
                    .iter()
                    .position(|c| *c == ch)
                    .unwrap_or_else(|| panic!("{ch} not found in {input:?}"));
                let offset = (pos > 0).then_some(pos).map_or(len / 2, |p| p / 2);

                if pos % 2 == 0 {
                    input.rotate_right(offset + len / 2 - 1 - pos);
                } else {
                    input.rotate_left(pos - offset);
                }
            }
            Self::MovePosition(x, y) => {
                let c = input.remove(y);
                input.insert(x, c);
            }
        }
    }
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                preceded(
                    tag("swap position "),
                    separated_pair(usize, tag(" with position "), usize),
                ),
                |(x, y)| Instruction::SwapPosition(x, y),
            ),
            map(
                preceded(
                    tag("swap letter "),
                    separated_pair(take(1u8), tag(" with letter "), take(1u8)),
                ),
                |(x, y): (&str, &str)| {
                    Instruction::SwapLetter(
                        x.chars()
                            .next()
                            .unwrap_or_else(|| unreachable!("Take 1 must not be empty")),
                        y.chars()
                            .next()
                            .unwrap_or_else(|| unreachable!("Take 1 must not be empty")),
                    )
                },
            ),
            map(
                (
                    alt((
                        value(false, tag("rotate left ")),
                        value(true, tag("rotate right ")),
                    )),
                    terminated(usize, alt((tag(" steps"), tag(" step")))),
                ),
                |(to_right, step)| {
                    if to_right {
                        Instruction::RotateRight(step)
                    } else {
                        Instruction::RotateLeft(step)
                    }
                },
            ),
            map(
                preceded(tag("rotate based on position of letter "), take(1u8)),
                |l: &str| {
                    Instruction::RotateBasedOnCharPosition(
                        l.chars()
                            .next()
                            .unwrap_or_else(|| unreachable!("Take 1 must not be empty")),
                    )
                },
            ),
            map(
                preceded(
                    tag("reverse positions "),
                    separated_pair(usize, tag(" through "), usize),
                ),
                |(x, y)| Instruction::ReversePositions(x, y),
            ),
            map(
                preceded(
                    tag("move position "),
                    separated_pair(usize, tag(" to position "), usize),
                ),
                |(x, y)| Instruction::MovePosition(x, y),
            ),
        ))
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        String::from_iter(self.instructions.iter().fold(
            "abcdefgh".chars().collect(),
            |mut input, instruction| {
                instruction.apply(&mut input);
                input
            },
        ))
    }

    fn part2(&self) -> String {
        String::from_iter(self.instructions.iter().rev().fold(
            "fbgdceah".chars().collect(),
            |mut input, instruction| {
                instruction.reverse(&mut input);
                input
            },
        ))
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
