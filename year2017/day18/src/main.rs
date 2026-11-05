use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        char,
        complete::{alpha1, isize, line_ending},
    },
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Register<T>(T);

#[derive(Clone, Copy)]
enum RegisterOrNumber<T> {
    Register(Register<T>),
    Number(isize),
}

impl<T> RegisterOrNumber<T> {
    fn cast<F, U>(self, op: F) -> RegisterOrNumber<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Self::Register(reg) => RegisterOrNumber::Register(Register(op(reg.0))),
            Self::Number(num) => RegisterOrNumber::Number(num),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction<T> {
    Snd(RegisterOrNumber<T>),
    Set(Register<T>, RegisterOrNumber<T>),
    Add(Register<T>, RegisterOrNumber<T>),
    Mul(Register<T>, RegisterOrNumber<T>),
    Mod(Register<T>, RegisterOrNumber<T>),
    Rcv(RegisterOrNumber<T>),
    Jgz(RegisterOrNumber<T>, RegisterOrNumber<T>),
}

macro_rules! cast_register_type {
    ($self:ident, $fn:ident, X $($sing:ident)+, X Y $($doub:ident)+, Y Y $($spec:ident)+) => {
        match $self {
            $(
                Self::$sing(ron) => Instruction::$sing(ron.cast($fn)),
            )+
            $(
                Self::$doub(Register(ra), ron) => {
                    Instruction::$doub(Register($fn(ra)), ron.cast($fn))
                }
            )+
            $(
                Self::$spec(ra, rb) => {
                    Instruction::$spec(ra.cast($fn), rb.cast($fn))
                }
            )+
        }
    };
}

impl<T> Instruction<T> {
    fn cast<F, U>(self, op: F) -> Instruction<U>
    where
        F: Copy + Fn(T) -> U,
    {
        cast_register_type!(self, op, X Snd Rcv, X Y Set Add Mul Mod, Y Y Jgz)
    }
}

#[derive(Default)]
struct Interpreter<'a> {
    registers: [isize; 32],
    ptr: usize,
    instructions: &'a [Instruction<usize>],
}

impl Interpreter<'_> {
    const fn get(&self, ron: RegisterOrNumber<usize>) -> isize {
        match ron {
            RegisterOrNumber::Number(num) => num,
            RegisterOrNumber::Register(reg) => self.registers[reg.0],
        }
    }

    fn execute<S, R>(&mut self, mut snd_cb: S, mut rcv_cb: R)
    where
        S: FnMut(isize),
        R: FnMut() -> Option<isize>,
    {
        while self.ptr < self.instructions.len() {
            match self.instructions[self.ptr] {
                Instruction::Snd(ron) => snd_cb(self.get(ron)),
                Instruction::Rcv(ron) => {
                    if let Some(val) = rcv_cb() {
                        if let RegisterOrNumber::Register(reg) = ron {
                            self.registers[reg.0] = val;
                        }
                    } else {
                        // No value returned means time to pause
                        break;
                    }
                }
                Instruction::Set(reg, ron) => {
                    self.registers[reg.0] = self.get(ron);
                }
                Instruction::Add(reg, ron) => {
                    self.registers[reg.0] += self.get(ron);
                }
                Instruction::Mul(reg, ron) => {
                    self.registers[reg.0] *= self.get(ron);
                }
                Instruction::Mod(reg, ron) => {
                    self.registers[reg.0] %= self.get(ron);
                }
                Instruction::Jgz(ra, rb) => {
                    if self.get(ra) > 0 {
                        self.ptr = self.ptr.wrapping_add_signed(self.get(rb));
                        continue;
                    }
                }
            }
            self.ptr += 1;
        }
    }
}

struct Puzzle {
    instructions: Vec<Instruction<usize>>,
    register_names: HashMap<&'static str, usize>,
}

impl Puzzle {
    fn parse_register(input: &'static str) -> IResult<&'static str, Register<&'static str>> {
        map(alpha1, Register).parse_complete(input)
    }

    fn parse_register_or_number(
        input: &'static str,
    ) -> IResult<&'static str, RegisterOrNumber<&'static str>> {
        alt((
            map(Self::parse_register, RegisterOrNumber::Register),
            map(isize, RegisterOrNumber::Number),
        ))
        .parse_complete(input)
    }

    fn parse_instruction(input: &'static str) -> IResult<&'static str, Instruction<&'static str>> {
        alt((
            map(
                preceded(tag("snd "), Self::parse_register_or_number),
                Instruction::Snd,
            ),
            map(
                preceded(tag("rcv "), Self::parse_register_or_number),
                Instruction::Rcv,
            ),
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
                    tag("add "),
                    separated_pair(
                        Self::parse_register,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Add(reg, val),
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
                    tag("mod "),
                    separated_pair(
                        Self::parse_register,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Mod(reg, val),
            ),
            map(
                preceded(
                    tag("jgz "),
                    separated_pair(
                        Self::parse_register_or_number,
                        char(' '),
                        Self::parse_register_or_number,
                    ),
                ),
                |(reg, val)| Instruction::Jgz(reg, val),
            ),
        ))
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_instruction))?;
        let names = instructions
            .iter()
            .flat_map(|&instruct| match instruct {
                Instruction::Snd(RegisterOrNumber::Register(reg))
                | Instruction::Rcv(RegisterOrNumber::Register(reg)) => vec![reg.0],
                Instruction::Add(ra, ron)
                | Instruction::Set(ra, ron)
                | Instruction::Mul(ra, ron)
                | Instruction::Mod(ra, ron) => match ron {
                    RegisterOrNumber::Register(rb) => vec![ra.0, rb.0],
                    RegisterOrNumber::Number(_) => vec![ra.0],
                },
                Instruction::Jgz(ra, rb) => match (ra, rb) {
                    (RegisterOrNumber::Register(ra), RegisterOrNumber::Register(rb)) => {
                        vec![ra.0, rb.0]
                    }
                    (RegisterOrNumber::Register(reg), _) | (_, RegisterOrNumber::Register(reg)) => {
                        vec![reg.0]
                    }
                    _ => vec![],
                },
                _ => vec![],
            })
            .fold(HashMap::new(), |mut names, n| {
                if !names.contains_key(n) {
                    names.insert(n, names.len());
                }
                names
            });
        let instructions = instructions
            .into_iter()
            .map(|instruct| instruct.cast(|name| names[name]))
            .collect::<Vec<_>>();
        Ok(Self {
            instructions,
            register_names: names,
        })
    }

    fn part1(&self) -> String {
        let mut t = Interpreter {
            instructions: &self.instructions,
            ..Default::default()
        };
        let mut last_played = None;
        t.execute(|val| last_played = Some(val), || None);
        let Some(last_played) = last_played else {
            panic!("No sound is ever played")
        };
        last_played.to_string()
    }

    fn part2(&self) -> String {
        let mut counter = 0;
        let mut channel_a = VecDeque::new();
        let mut channel_b = VecDeque::new();
        let mut thread_a = Interpreter {
            instructions: &self.instructions,
            ..Default::default()
        };
        let mut thread_b = Interpreter {
            instructions: &self.instructions,
            ..Default::default()
        };
        thread_b.registers[self.register_names["p"]] = 1;
        loop {
            // Execute A first, then B. The order doesn't matter.
            thread_a.execute(|val| channel_b.push_back(val), || channel_a.pop_front());
            thread_b.execute(
                |val| {
                    counter += 1;
                    channel_a.push_back(val);
                },
                || channel_b.pop_front(),
            );
            // The specific puzzle input would only result in a deadlock,
            // no other ways of termination is possible
            if channel_a.is_empty() && channel_b.is_empty() {
                break;
            }
        }
        counter.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
