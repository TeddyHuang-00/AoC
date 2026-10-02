use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, PartialEq)]
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
    Copy(RegisterOrNumber, RegisterOrNumber),
    Increase(Register),
    Decrease(Register),
    JumpNotZero(RegisterOrNumber, RegisterOrNumber),
    Toggle(Register),
}

impl Instruction {
    const fn toggle(&mut self) -> Self {
        *self = match *self {
            Self::Increase(r) => Self::Decrease(r),
            Self::Decrease(r) | Self::Toggle(r) => Self::Increase(r),
            Self::Copy(a, b) => Self::JumpNotZero(a, b),
            Self::JumpNotZero(a, b) => Self::Copy(a, b),
        };

        *self
    }
}

#[derive(Clone, Copy, Default)]
struct Registers {
    registers: [i32; 4],
}

impl Registers {
    const fn get(&self, r: Register) -> i32 {
        self.registers[r as usize]
    }

    const fn get_or_number(&self, rnn: RegisterOrNumber) -> i32 {
        match rnn {
            RegisterOrNumber::Register(r) => self.registers[r as usize],
            RegisterOrNumber::Number(n) => n,
        }
    }

    const fn set(&mut self, r: Register, n: i32) {
        self.registers[r as usize] = n;
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
                        Self::parse_register_or_number,
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
                    separated_pair(
                        Self::parse_register_or_number,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(x, y)| Instruction::JumpNotZero(x, y),
            ),
            map(preceded(tag("tgl "), Self::parse_register), |x| {
                Instruction::Toggle(x)
            }),
        ))
        .parse_complete(input)
    }

    fn try_add_sub(
        instructions: [Instruction; 3],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                a,
                b,
                Instruction::JumpNotZero(
                    RegisterOrNumber::Register(cond),
                    RegisterOrNumber::Number(-2),
                ),
            ] if matches!(b, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                && matches!(a, Instruction::Increase(_) | Instruction::Decrease(_))
                || matches!(a, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                    && matches!(b, Instruction::Decrease(_) | Instruction::Increase(_)) =>
            {
                let num_loops = registers.get(cond).abs();
                registers.set(cond, 0);

                let other = if matches!(a, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                {
                    b
                } else {
                    a
                };
                match other {
                    Instruction::Increase(r) => registers.set(r, registers.get(r) + num_loops),
                    Instruction::Decrease(r) => registers.set(r, registers.get(r) - num_loops),
                    _ => unreachable!(),
                }

                *ptr += instructions.len();
                true
            }
            _ => {
                // Do nothing.
                false
            }
        }
    }

    fn try_multiply(
        instructions: [Instruction; 6],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                reset,
                a,
                b,
                Instruction::JumpNotZero(
                    RegisterOrNumber::Register(cond),
                    RegisterOrNumber::Number(-2),
                ),
                c,
                Instruction::JumpNotZero(
                    RegisterOrNumber::Register(other),
                    RegisterOrNumber::Number(-5),
                ),
            ] if matches!(b, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                && matches!(a, Instruction::Increase(_) | Instruction::Decrease(_))
                && matches!(reset, Instruction::Copy(_, RegisterOrNumber::Register(r)) if cond == r)
                && matches!(c, Instruction::Decrease(r) | Instruction::Increase(r) if other == r) =>
            {
                let inner = match reset {
                    Instruction::Copy(src, _) => registers.get_or_number(src),
                    _ => unreachable!(),
                };
                let outer = registers.get(other).abs();
                registers.set(cond, 0);
                registers.set(other, 0);
                match a {
                    Instruction::Increase(r) => registers.set(r, registers.get(r) + inner * outer),
                    Instruction::Decrease(r) => registers.set(r, registers.get(r) - inner * outer),
                    _ => unreachable!(),
                }

                *ptr += instructions.len();
                true
            }
            _ => {
                // Do nothing.
                false
            }
        }
    }

    fn try_optimized_loop(
        instructions: &[Instruction],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            ins if ins.len() == 6 => Self::try_multiply(
                instructions.try_into().unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            ins if ins.len() == 5 => Self::try_add_sub(
                instructions.try_into().unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            _ => false,
        }
    }

    fn execute(&self, mut registers: Registers) -> Registers {
        let mut instructions = self.instructions.clone();
        let mut ptr = 0;
        'step: while ptr < instructions.len() {
            // The code will run painfully slow without the for loop
            // optimizations.
            for len in [6, 3] {
                if ptr + len <= instructions.len()
                    && Self::try_optimized_loop(
                        instructions[ptr..ptr + len]
                            .try_into()
                            .unwrap_or_else(|_| unreachable!()),
                        &mut registers,
                        &mut ptr,
                    )
                {
                    continue 'step;
                }
            }

            match instructions[ptr] {
                Instruction::Copy(x, RegisterOrNumber::Register(y)) => {
                    registers.set(y, registers.get_or_number(x));
                }
                Instruction::Increase(x) => registers.inc(x),
                Instruction::Decrease(x) => registers.dec(x),
                Instruction::JumpNotZero(x, y) => {
                    if registers.get_or_number(x) != 0 {
                        ptr = ptr.wrapping_add_signed(registers.get_or_number(y) as isize);
                        continue;
                    }
                }
                Instruction::Toggle(x) => {
                    let ptr = ptr.wrapping_add_signed(registers.get(x) as isize);
                    if ptr < instructions.len() {
                        instructions[ptr].toggle();
                    }
                }
                Instruction::Copy(_, _) => {
                    // Invalid copy instruction, just do nothing
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
        let registers = self.execute(Registers {
            registers: [7, 0, 0, 0],
        });
        registers.get(Register::A).to_string()
    }

    fn part2(&self) -> String {
        let registers = self.execute(Registers {
            registers: [12, 0, 0, 0],
        });
        registers.get(Register::A).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
