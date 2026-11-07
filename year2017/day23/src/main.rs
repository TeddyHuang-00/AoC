use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, isize, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

macro_rules! all_same {
    ($left:ident, $right:ident, $($vars:ident),+) => {
        all_same!($left, $right) && all_same!($right, $($vars),+)
    };
    ($left:ident, $right:ident) => {
        $left == $right
    };
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Register(u8);

#[derive(Clone, Copy, Eq, PartialEq)]
enum RegisterOrNumber {
    Register(Register),
    Number(isize),
}

#[derive(Clone, Copy)]
enum Instruction {
    Set(Register, RegisterOrNumber),
    Sub(Register, RegisterOrNumber),
    Mul(Register, RegisterOrNumber),
    Jnz(RegisterOrNumber, RegisterOrNumber),
}

#[derive(Default)]
struct Interpreter<'a> {
    registers: [isize; 8],
    ptr: usize,
    instructions: &'a [Instruction],
}

impl Interpreter<'_> {
    fn at(&self, reg: Register) -> isize {
        self.registers[usize::from(reg.0)]
    }

    fn at_mut(&mut self, reg: Register) -> &mut isize {
        &mut self.registers[usize::from(reg.0)]
    }

    fn get(&self, ron: RegisterOrNumber) -> isize {
        match ron {
            RegisterOrNumber::Number(num) => num,
            RegisterOrNumber::Register(reg) => self.at(reg),
        }
    }

    fn try_fast_path(&mut self, instructions: &[Instruction; 24]) -> bool {
        use Instruction::{Jnz, Mul, Set, Sub};
        use RegisterOrNumber::{Number, Register};
        match instructions {
            &[
                Set(cond, Number(always1)),
                Set(outer, outer_reset),
                Set(inner, inner_reset),
                Set(tmp, Register(o1)),
                Mul(t1, Register(i1)),
                Sub(t2, Register(var)),
                Jnz(Register(t3), Number(2)),
                Set(c1, Number(0)),
                Sub(i2, Number(-1)),
                Set(t4, Register(i3)),
                Sub(t5, Register(v1)),
                Jnz(Register(t6), Number(-8)),
                Sub(o2, Number(-1)),
                Set(t7, Register(o3)),
                Sub(t8, Register(v2)),
                Jnz(Register(t9), Number(-13)),
                Jnz(Register(c2), Number(2)),
                Sub(out, val),
                Set(t10, Register(v3)),
                Sub(t11, Register(tgt)),
                Jnz(Register(t12), Number(2)),
                Jnz(Number(always2), Number(3)),
                Sub(v4, step),
                Jnz(Number(always3), Number(-23)),
            ] if all_same!(tmp, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12)
                && all_same!(inner, i1, i2, i3)
                && all_same!(var, v1, v2, v3, v4)
                && all_same!(outer, o1, o2, o3)
                && all_same!(cond, c1, c2)
                && always1 != 0
                && always2 != 0
                && always3 != 0 =>
            {
                let outer_reset = self.get(outer_reset);
                let inner_reset = self.get(inner_reset);
                let start = self.at(var);
                let end = self.at(tgt);
                let step_size = self.get(step);
                let candidates = if step_size < 0 {
                    (start..=end).step_by(step_size.unsigned_abs())
                } else {
                    (end..=start).step_by(step_size.cast_unsigned())
                };
                *self.at_mut(out) -= candidates
                    .filter(|&v| {
                        (outer_reset..v)
                            .take_while(|f| f * f <= v)
                            .any(|f| v % f == 0 && v / f >= inner_reset)
                    })
                    .count()
                    .cast_signed()
                    * self.get(val);
                // These are not used, but we are writing an interpreter,
                // so just to make it correct...
                *self.at_mut(tmp) = 0;
                *self.at_mut(var) = end;
                *self.at_mut(inner) = end;
                *self.at_mut(outer) = end;
                // This is the result from last step
                *self.at_mut(cond) = if (outer_reset..end)
                    .take_while(|f| f * f <= end)
                    .any(|f| end % f == 0 && end / f >= inner_reset)
                {
                    0
                } else {
                    always1
                };
                self.ptr += 24;
                true
            }
            _ => false,
        }
    }

    fn execute<F>(&mut self, mut debug_callback: Option<F>)
    where
        F: FnMut(Instruction),
    {
        while self.ptr < self.instructions.len() {
            if debug_callback.is_none()
                && self.ptr + 24 <= self.instructions.len()
                && self.try_fast_path(
                    self.instructions[self.ptr..self.ptr + 24]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                )
            {
                continue;
            }
            match self.instructions[self.ptr] {
                Instruction::Set(reg, ron) => {
                    self.registers[usize::from(reg.0)] = self.get(ron);
                }
                Instruction::Sub(reg, ron) => {
                    self.registers[usize::from(reg.0)] -= self.get(ron);
                }
                Instruction::Mul(reg, ron) => {
                    self.registers[usize::from(reg.0)] *= self.get(ron);
                }
                Instruction::Jnz(ra, rb) => {
                    if self.get(ra) != 0 {
                        self.ptr = self.ptr.wrapping_add_signed(self.get(rb));
                        continue;
                    }
                }
            }
            if let Some(cb) = debug_callback.as_mut() {
                cb(self.instructions[self.ptr]);
            }
            self.ptr += 1;
        }
    }
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn parse_register(input: &str) -> IResult<&str, Register> {
        map_res(alpha1, |s: &str| {
            s.chars()
                .next()
                .ok_or_else(|| unreachable!("Must take one char"))
                .map(|ch| Register((ch as u8) - b'a'))
        })
        .parse_complete(input)
    }

    fn parse_register_or_number(input: &str) -> IResult<&str, RegisterOrNumber> {
        alt((
            map(Self::parse_register, RegisterOrNumber::Register),
            map(isize, RegisterOrNumber::Number),
        ))
        .parse_complete(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                preceded(
                    tag("set "),
                    separated_pair(
                        Self::parse_register,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Set(reg, val),
            ),
            map(
                preceded(
                    tag("sub "),
                    separated_pair(
                        Self::parse_register,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Sub(reg, val),
            ),
            map(
                preceded(
                    tag("mul "),
                    separated_pair(
                        Self::parse_register,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Mul(reg, val),
            ),
            map(
                preceded(
                    tag("jnz "),
                    separated_pair(
                        Self::parse_register_or_number,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Jnz(reg, val),
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
        let mut interpreter = Interpreter {
            instructions: &self.instructions,
            ..Default::default()
        };
        let mut counter = 0;
        interpreter.execute(Some(|ins| {
            if matches!(ins, Instruction::Mul(_, _)) {
                counter += 1;
            }
        }));
        counter.to_string()
    }

    fn part2(&self) -> String {
        let mut interpreter = Interpreter {
            instructions: &self.instructions,
            ..Default::default()
        };
        *interpreter.at_mut(Register(0)) = 1;
        interpreter.execute::<fn(Instruction)>(None);
        interpreter.at(Register(7)).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
