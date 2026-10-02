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
    Output(Register),
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
            map(preceded(tag("out "), Self::parse_register), |x| {
                Instruction::Output(x)
            }),
        ))
        .parse_complete(input)
    }

    fn try_subtract(
        instructions: [Instruction; 5],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                Instruction::JumpNotZero(RegisterOrNumber::Register(sub), 2),
                Instruction::JumpNotZero(RegisterOrNumber::Number(brk), 4),
                Instruction::Decrease(left),
                Instruction::Decrease(right),
                Instruction::JumpNotZero(RegisterOrNumber::Number(lp), -4),
            ] if sub == right && brk != 0 && lp != 0 => {
                registers.set(left, registers.get(left) - registers.get(right));
                registers.set(right, 0);
                *ptr += 5;
                true
            }
            _ => false,
        }
    }

    fn try_multiply(
        instructions: [Instruction; 6],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                Instruction::Copy(src, init),
                out,
                inner_update,
                Instruction::JumpNotZero(RegisterOrNumber::Register(inner_cond), -2),
                outer_update,
                Instruction::JumpNotZero(RegisterOrNumber::Register(outer_cond), -5),
            ] if init == inner_cond
                && matches!(inner_update, Instruction::Decrease(cond) | Instruction::Increase(cond) if cond == inner_cond)
                && matches!(outer_update, Instruction::Decrease(cond) | Instruction::Increase(cond) if cond == outer_cond)
                && matches!(out, Instruction::Decrease(_) | Instruction::Increase(_)) =>
            {
                let inner = registers.get_or_number(src);
                let outer = registers.get(outer_cond);

                match out {
                    Instruction::Decrease(r) => {
                        registers.set(r, registers.get(r) - inner * outer);
                    }
                    Instruction::Increase(r) => {
                        registers.set(r, registers.get(r) + inner * outer);
                    }
                    _ => unreachable!(),
                }

                registers.set(inner_cond, 0);
                registers.set(outer_cond, 0);

                *ptr += 6;
                true
            }
            _ => false,
        }
    }

    fn try_modulo(
        instructions: [Instruction; 8],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            [
                Instruction::Copy(modu, reset),
                Instruction::JumpNotZero(RegisterOrNumber::Register(divid), 2),
                Instruction::JumpNotZero(RegisterOrNumber::Number(brk), 6),
                Instruction::Decrease(reg_divid),
                inner_update,
                Instruction::JumpNotZero(RegisterOrNumber::Register(reg_modu), -4),
                out,
                Instruction::JumpNotZero(RegisterOrNumber::Number(lp), -7),
            ] if brk != 0
                && lp != 0
                && reset == reg_modu
                && matches!(inner_update, Instruction::Decrease(r) | Instruction::Increase(r) if r == reg_modu)
                && reg_divid == divid
                && matches!(out, Instruction::Decrease(r) | Instruction::Increase(r) if r != reg_modu && r != reg_divid) =>
            {
                let dividend = registers.get(divid);
                let modulus = registers.get_or_number(modu);

                // The result of the division will be stored in the out register
                match out {
                    Instruction::Decrease(r) => {
                        registers.set(r, registers.get(r) - dividend / modulus);
                    }
                    Instruction::Increase(r) => {
                        registers.set(r, registers.get(r) + dividend / modulus);
                    }
                    _ => unreachable!(),
                }

                // The difference between the modulus and the remainder will be
                // stored in the inner_cond register.
                registers.set(reg_modu, modulus - dividend % modulus);
                registers.set(reg_divid, 0);

                *ptr += 8;
                true
            }
            _ => false,
        }
    }

    fn try_optimized_loop(
        instructions: &[Instruction],
        registers: &mut Registers,
        ptr: &mut usize,
    ) -> bool {
        match instructions {
            ins if ins.len() == 8 => Self::try_modulo(
                <[_; 8]>::try_from(instructions).unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            ins if ins.len() == 6 => Self::try_multiply(
                <[_; 6]>::try_from(instructions).unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            ins if ins.len() == 5 => Self::try_subtract(
                <[_; 5]>::try_from(instructions).unwrap_or_else(|_| unreachable!()),
                registers,
                ptr,
            ),
            _ => false,
        }
    }

    fn execute(&self, mut registers: Registers) -> bool {
        let mut output = 1;
        let mut cnt = 0;
        let mut ptr = 0;

        'step: while ptr < self.instructions.len() {
            for len in [8, 6, 5] {
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
                Instruction::Output(x) => {
                    let out = registers.get(x);
                    // output must be alternating 1/0
                    if output + out != 1 {
                        return false;
                    }
                    output = out;
                    // We just need to check a certain number of loops to
                    // determine if the program is valid
                    cnt += 1;
                    if cnt >= 16 {
                        return true;
                    }
                }
            }
            ptr += 1;
        }

        unreachable!("The program should be infinite loop")
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        for i in 0.. {
            let regs = Registers {
                registers: [i, 0, 0, 0],
            };
            if self.execute(regs) {
                return i.to_string();
            }
        }
        unreachable!("No valid input found")
    }

    fn part2(&self) -> String {
        String::default()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
