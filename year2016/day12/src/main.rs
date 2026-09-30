use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, isize, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy)]
enum RegisterOrNumber {
    Register(Register),
    Number(i32),
}

#[derive(Clone, Copy)]
enum Instruction {
    Copy(RegisterOrNumber, Register),
    Increase(Register),
    Decrease(Register),
    JumpNotZero(RegisterOrNumber, isize),
}

#[derive(Clone, Copy, Default)]
struct Registers {
    registers: [i32; 4],
}

impl Registers {
    const fn get(&self, rnn: RegisterOrNumber) -> i32 {
        match rnn {
            RegisterOrNumber::Register(r) => self.registers[r as usize],
            RegisterOrNumber::Number(n) => n,
        }
    }

    const fn set(&mut self, r: Register, rnn: RegisterOrNumber) {
        self.registers[r as usize] = self.get(rnn);
    }

    const fn inc(&mut self, r: Register) {
        self.registers[r as usize] += 1;
    }

    const fn dec(&mut self, r: Register) {
        self.registers[r as usize] -= 1;
    }
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_register(input: &str) -> IResult<&str, Register> {
        alt((
            value(Register::A, char('a')),
            value(Register::B, char('b')),
            value(Register::C, char('c')),
            value(Register::D, char('d')),
        ))
        .parse_complete(input)
    }

    fn parse_register_or_number(input: &str) -> IResult<&str, RegisterOrNumber> {
        alt((
            map(Self::parse_register, RegisterOrNumber::Register),
            map(i32, RegisterOrNumber::Number),
        ))
        .parse_complete(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                preceded(
                    tag("cpy "),
                    separated_pair(
                        Self::parse_register_or_number,
                        char(' '),
                        Self::parse_register,
                    ),
                ),
                |(x, y)| Instruction::Copy(x, y),
            ),
            map(preceded(tag("inc "), Self::parse_register), |x| {
                Instruction::Increase(x)
            }),
            map(preceded(tag("dec "), Self::parse_register), |x| {
                Instruction::Decrease(x)
            }),
            map(
                preceded(
                    tag("jnz "),
                    separated_pair(Self::parse_register_or_number, char(' '), isize),
                ),
                |(x, y)| Instruction::JumpNotZero(x, y),
            ),
        ))
        .parse_complete(input)
    }

    fn execute(&self, mut registers: Registers) -> Registers {
        let mut ptr = 0;

        while ptr < self.instructions.len() {
            match self.instructions[ptr] {
                Instruction::Copy(x, y) => registers.set(y, x),
                Instruction::Increase(x) => registers.inc(x),
                Instruction::Decrease(x) => registers.dec(x),
                Instruction::JumpNotZero(x, y) => {
                    if registers.get(x) != 0 {
                        ptr = ptr.wrapping_add_signed(y);
                        continue;
                    }
                }
            }
            ptr += 1;
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
        let registers = self.execute(Registers::default());
        registers
            .get(RegisterOrNumber::Register(Register::A))
            .to_string()
    }

    fn part2(&self) -> String {
        let mut registers = Registers::default();
        registers.set(Register::C, RegisterOrNumber::Number(1));
        registers = self.execute(registers);
        registers
            .get(RegisterOrNumber::Register(Register::A))
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
