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
    const fn get(&self, r: Register) -> i32 {
        self.registers[r as usize]
    }

    const fn get_or_number(&self, rnn: RegisterOrNumber) -> i32 {
        match rnn {
            RegisterOrNumber::Register(r) => self.get(r),
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

    fn try_add_sub(
        instructions: [Instruction; 3],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                a,
                b,
                Instruction::JumpNotZero(RegisterOrNumber::Register(cond), -2),
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

                *ptr += 3;
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
                Instruction::JumpNotZero(RegisterOrNumber::Register(cond), -2),
                c,
                Instruction::JumpNotZero(RegisterOrNumber::Register(other), -5),
            ] if matches!(b, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                && matches!(a, Instruction::Increase(_) | Instruction::Decrease(_))
                && matches!(reset, Instruction::Copy(_, r) if cond == r)
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

                *ptr += 6;
                true
            }
            _ => {
                // Do nothing.
                false
            }
        }
    }

    fn try_multiply_with_leftover(
        instructions: [Instruction; 7],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                reset,
                a,
                b,
                Instruction::JumpNotZero(RegisterOrNumber::Register(cond), -2),
                copy,
                c,
                Instruction::JumpNotZero(RegisterOrNumber::Register(other), -5),
            ] if matches!(b, Instruction::Decrease(r) | Instruction::Increase(r) if cond == r)
                && matches!(a, Instruction::Increase(_) | Instruction::Decrease(_))
                && matches!((reset, copy), (Instruction::Copy(_, r1), Instruction::Copy(RegisterOrNumber::Register(r2), r3)) if cond == r3 && r1 == r2)
                && matches!(c, Instruction::Decrease(r) | Instruction::Increase(r) if other == r) =>
            {
                let inner = match reset {
                    Instruction::Copy(src, dst) => {
                        let val = registers.get_or_number(src);
                        registers.set(dst, val);
                        val
                    }
                    _ => unreachable!(),
                };
                let outer = registers.get(other).abs();
                registers.set(cond, inner);
                registers.set(other, 0);
                match a {
                    Instruction::Increase(r) => registers.set(r, registers.get(r) + inner * outer),
                    Instruction::Decrease(r) => registers.set(r, registers.get(r) - inner * outer),
                    _ => unreachable!(),
                }

                *ptr += 7;
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
            ins if ins.len() == 7 => Self::try_multiply_with_leftover(
                instructions.try_into().unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            ins if ins.len() == 6 => Self::try_multiply(
                instructions.try_into().unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            ins if ins.len() == 3 => Self::try_add_sub(
                instructions.try_into().unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            _ => false,
        }
    }

    fn execute(&self, mut registers: Registers) -> Registers {
        let mut ptr = 0;

        'step: while ptr < self.instructions.len() {
            for len in [7, 6, 3] {
                if ptr + len <= self.instructions.len()
                    && Self::try_optimized_loop(
                        &self.instructions[ptr..ptr + len],
                        &mut registers,
                        &mut ptr,
                    )
                {
                    continue 'step;
                }
            }

            match self.instructions[ptr] {
                Instruction::Copy(x, y) => registers.set(y, registers.get_or_number(x)),
                Instruction::Increase(x) => registers.inc(x),
                Instruction::Decrease(x) => registers.dec(x),
                Instruction::JumpNotZero(x, y) => {
                    if registers.get_or_number(x) != 0 {
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
        registers.get(Register::A).to_string()
    }

    fn part2(&self) -> String {
        let mut registers = Registers::default();
        registers.set(Register::C, 1);
        registers = self.execute(registers);
        registers.get(Register::A).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
