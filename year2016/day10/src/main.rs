use std::collections::BTreeMap;

use anyhow::Result;
use itertools::{
    Either::{Left, Right},
    Itertools,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, usize},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Bot(usize);

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Output(usize);

#[derive(Clone, Copy)]
enum Target {
    Output(Output),
    Bot(Bot),
}

#[derive(Clone, Copy)]
enum Instruction {
    Input { value: usize, bot: Bot },
    Compare { bot: Bot, low: Target, high: Target },
}

struct Puzzle {
    targets: BTreeMap<Bot, (Target, Target)>,
    initial: BTreeMap<Bot, Vec<usize>>,
}

impl Puzzle {
    fn parse_value(input: &str) -> IResult<&str, usize> {
        preceded(tag("value "), usize).parse_complete(input)
    }

    fn parse_bot(input: &str) -> IResult<&str, Bot> {
        map(preceded(tag("bot "), usize), Bot).parse_complete(input)
    }

    fn parse_output(input: &str) -> IResult<&str, Output> {
        map(preceded(tag("output "), usize), Output).parse_complete(input)
    }

    fn parse_target(input: &str) -> IResult<&str, Target> {
        alt((
            map(Self::parse_bot, Target::Bot),
            map(Self::parse_output, Target::Output),
        ))
        .parse_complete(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                separated_pair(Self::parse_value, tag(" goes to "), Self::parse_bot),
                |(value, bot)| Instruction::Input { value, bot },
            ),
            map(
                separated_pair(
                    Self::parse_bot,
                    tag(" gives low to "),
                    separated_pair(Self::parse_target, tag(" and high to "), Self::parse_target),
                ),
                |(bot, (low, high))| Instruction::Compare { bot, low, high },
            ),
        ))
        .parse_complete(input)
    }

    fn process(&self, part1: bool) -> String {
        let mut bots = self.initial.clone();
        let mut outputs = BTreeMap::new();
        let mut frontiers = bots
            .iter()
            .filter_map(|(&bot, chips)| (chips.len() >= 2).then_some(bot))
            .collect::<Vec<_>>();
        while let Some(bot) = frontiers.pop() {
            let chips = &bots[&bot];
            let (low, high) = self.targets[&bot];
            let (l, h) = (chips[0].min(chips[1]), chips[0].max(chips[1]));
            if part1 && l == 17 && h == 61 {
                return bot.0.to_string();
            }

            match low {
                Target::Bot(b) => {
                    bots.entry(b)
                        .and_modify(|chips| {
                            if chips.len() == 1 {
                                frontiers.push(b);
                            }
                            chips.push(l);
                        })
                        .or_insert_with(|| vec![l]);
                }
                Target::Output(o) => {
                    outputs.insert(o, l);
                }
            }
            match high {
                Target::Bot(b) => {
                    bots.entry(b)
                        .and_modify(|chips| {
                            if chips.len() == 1 {
                                frontiers.push(b);
                            }
                            chips.push(h);
                        })
                        .or_insert_with(|| vec![h]);
                }
                Target::Output(o) => {
                    outputs.insert(o, h);
                }
            }

            bots.entry(bot).and_modify(std::vec::Vec::clear);
        }
        if !part1
            && outputs.contains_key(&Output(0))
            && outputs.contains_key(&Output(1))
            && outputs.contains_key(&Output(2))
        {
            return (0..=2)
                .into_iter()
                .map(|o| outputs[&Output(o)])
                .product::<usize>()
                .to_string();
        }
        panic!("No result found")
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        let (targets, inputs): (BTreeMap<_, _>, Vec<_>) =
            instructions
                .iter()
                .partition_map(|&instruction| match instruction {
                    Instruction::Compare { bot, low, high } => Left((bot, (low, high))),
                    Instruction::Input { value, bot } => Right((bot, value)),
                });
        let initial = inputs
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, (bot, chip)| {
                acc.entry(bot)
                    .and_modify(|chips: &mut Vec<usize>| chips.push(chip))
                    .or_insert_with(|| vec![chip]);
                acc
            });
        Ok(Self { targets, initial })
    }

    fn part1(&self) -> String {
        self.process(true)
    }

    fn part2(&self) -> String {
        self.process(false)
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
