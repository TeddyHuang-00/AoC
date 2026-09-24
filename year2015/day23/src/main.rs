use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, isize, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl Register {
    const fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_register(input: &str) -> IResult<&str, Register> {
        alt((value(Register::A, char('a')), value(Register::B, char('b')))).parse_complete(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("hlf "), Self::parse_register), |register| {
                Instruction::Half(register)
            }),
            map(preceded(tag("tpl "), Self::parse_register), |register| {
                Instruction::Triple(register)
            }),
            map(preceded(tag("inc "), Self::parse_register), |register| {
                Instruction::Increment(register)
            }),
            map(preceded(tag("jmp "), isize), |offset| {
                Instruction::Jump(offset)
            }),
            map(
                preceded(
                    tag("jie "),
                    separated_pair(Self::parse_register, tag(", "), isize),
                ),
                |(register, offset)| Instruction::JumpIfEven(register, offset),
            ),
            map(
                preceded(
                    tag("jio "),
                    separated_pair(Self::parse_register, tag(", "), isize),
                ),
                |(register, offset)| Instruction::JumpIfOne(register, offset),
            ),
        ))
        .parse_complete(input)
    }

    fn turing_machine(&self, init: [usize; 2]) -> [usize; 2] {
        let mut registers = init;
        let mut idx = 0;
        while idx < self.instructions.len() {
            match self.instructions[idx] {
                Instruction::Half(r) => registers[r.idx()] /= 2,
                Instruction::Triple(r) => registers[r.idx()] *= 3,
                Instruction::Increment(r) => registers[r.idx()] += 1,
                Instruction::Jump(o) => {
                    idx = idx.wrapping_add_signed(o);
                    continue;
                }
                Instruction::JumpIfEven(r, o) => {
                    if registers[r.idx()].is_multiple_of(2) {
                        idx = idx.wrapping_add_signed(o);
                        continue;
                    }
                }
                Instruction::JumpIfOne(r, o) => {
                    if registers[r.idx()] == 1 {
                        idx = idx.wrapping_add_signed(o);
                        continue;
                    }
                }
            }
            idx += 1;
        }
        registers
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        self.turing_machine([0; 2])[Register::B.idx()].to_string()
    }

    fn part2(&self) -> String {
        self.turing_machine([1, 0])[Register::B.idx()].to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
