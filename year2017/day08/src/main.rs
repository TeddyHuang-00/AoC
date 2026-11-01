use std::{collections::BTreeMap, panic};

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::separated_pair,
};
use util::{Solution, chore, parser};

type RegisterValuePair<T> = (T, i32);
type Instruction<T> = ((Action, RegisterValuePair<T>), (Rule, RegisterValuePair<T>));

#[derive(Clone, Copy)]
enum Action {
    Inc,
    Dec,
}

impl Action {
    const fn sign(self) -> i32 {
        match self {
            Self::Dec => -1,
            Self::Inc => 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Rule {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Neq,
}

impl Rule {
    const fn satisfy(self, lhs: i32, rhs: i32) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Neq => lhs != rhs,
            Self::Ge => lhs >= rhs,
            Self::Gt => lhs > rhs,
            Self::Le => lhs <= rhs,
            Self::Lt => lhs < rhs,
        }
    }
}

struct Puzzle {
    instructions: Vec<Instruction<usize>>,
    names: BTreeMap<usize, &'static str>,
}

impl Puzzle {
    fn parse_instruction(input: &'static str) -> IResult<&'static str, Instruction<&'static str>> {
        separated_pair(
            map(
                (
                    alpha1,
                    alt((
                        value(Action::Inc, tag(" inc ")),
                        value(Action::Dec, tag(" dec ")),
                    )),
                    i32,
                ),
                |(reg, act, val)| (act, (reg, val)),
            ),
            tag(" if "),
            map(
                (
                    alpha1,
                    alt((
                        value(Rule::Eq, tag(" == ")),
                        value(Rule::Neq, tag(" != ")),
                        value(Rule::Ge, tag(" >= ")),
                        value(Rule::Gt, tag(" > ")),
                        value(Rule::Le, tag(" <= ")),
                        value(Rule::Lt, tag(" < ")),
                    )),
                    i32,
                ),
                |(reg, rul, val)| (rul, (reg, val)),
            ),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        let names = instructions
            .iter()
            .flat_map(|&((_, (a, _)), (_, (b, _)))| [a, b])
            .fold(BTreeMap::new(), |mut acc, x| {
                if !acc.contains_key(x) {
                    acc.insert(x, acc.len());
                }
                acc
            });
        let instructions = instructions
            .into_iter()
            .map(|((act, (ar, av)), (rul, (rr, rv)))| {
                ((act, (names[ar], av)), (rul, (names[rr], rv)))
            })
            .collect();
        let names = names.into_iter().map(|(name, id)| (id, name)).collect();
        Ok(Self {
            instructions,
            names,
        })
    }

    fn part1(&self) -> String {
        let mut registers = vec![0; self.names.len()];
        for &((act, (ar, av)), (rul, (rr, rv))) in &self.instructions {
            if rul.satisfy(registers[rr], rv) {
                registers[ar] += act.sign() * av;
            }
        }
        registers
            .into_iter()
            .max()
            .unwrap_or_else(|| panic!("Registers must not be empty"))
            .to_string()
    }

    fn part2(&self) -> String {
        let mut registers = vec![0; self.names.len()];
        let mut max = 0;
        for &((act, (ar, av)), (rul, (rr, rv))) in &self.instructions {
            if rul.satisfy(registers[rr], rv) {
                registers[ar] += act.sign() * av;
                max = max.max(registers[ar]);
            }
        }
        max.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
