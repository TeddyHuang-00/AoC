use std::{
    collections::{BTreeMap, HashSet},
    hash::BuildHasherDefault,
};

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, char, line_ending, space1},
        streaming::usize,
    },
    combinator::{map, value},
    multi::{many_m_n, separated_list1},
    sequence::{delimited, preceded, separated_pair},
};
use util::{Solution, chore, hasher::FastHasher, parser};

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
struct MachineState(u8);

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Rule {
    state: MachineState,
    value: bool,
}

#[derive(Clone, Copy, Default)]
struct Action {
    write: bool,
    move_to_right: bool,
    next_state: MachineState,
}

type RuleSet = BTreeMap<Rule, Action>;

struct Puzzle {
    init_state: MachineState,
    diagnostic_steps: usize,
    rulesets: RuleSet,
}

impl Puzzle {
    fn parse_state(input: &str) -> IResult<&str, MachineState> {
        map(preceded(tag("state "), alpha1), |s: &str| {
            let ch = s.chars().next().unwrap_or_else(|| unreachable!());
            MachineState(ch as u8 - b'A')
        })
        .parse_complete(input)
    }

    fn parse_value(input: &str) -> IResult<&str, bool> {
        alt((value(true, char('1')), value(false, char('0')))).parse_complete(input)
    }

    fn parse_heading(input: &str) -> IResult<&str, (MachineState, usize)> {
        separated_pair(
            delimited(tag("Begin in "), Self::parse_state, tag(".")),
            line_ending,
            delimited(
                tag("Perform a diagnostic checksum after "),
                usize,
                tag(" steps."),
            ),
        )
        .parse_complete(input)
    }

    fn parse_state_ruleset(input: &str) -> IResult<&str, [(Rule, Action); 2]> {
        map(
            (
                delimited(tag("In "), Self::parse_state, (tag(":"), line_ending)),
                many_m_n(
                    2,
                    2,
                    (
                        delimited(
                            (space1, tag("If the current value is ")),
                            Self::parse_value,
                            (tag(":"), line_ending),
                        ),
                        delimited(
                            (space1, tag("- Write the value ")),
                            Self::parse_value,
                            (tag("."), line_ending),
                        ),
                        delimited(
                            (space1, tag("- Move one slot to the ")),
                            alt((value(true, tag("right")), value(false, tag("left")))),
                            (tag("."), line_ending),
                        ),
                        delimited(
                            (space1, tag("- Continue with ")),
                            Self::parse_state,
                            (tag("."), line_ending),
                        ),
                    ),
                ),
            ),
            |(state, rules)| {
                let mut ruleset = [Default::default(); 2];
                for (idx, (value, write, move_to_right, next_state)) in
                    rules.into_iter().take(2).enumerate()
                {
                    ruleset[idx] = (
                        Rule { state, value },
                        Action {
                            write,
                            move_to_right,
                            next_state,
                        },
                    );
                }
                ruleset
            },
        )
        .parse_complete(input)
    }

    fn parse_turing_machine(input: &str) -> IResult<&str, ((MachineState, usize), RuleSet)> {
        separated_pair(
            Self::parse_heading,
            (line_ending, line_ending),
            map(
                separated_list1(line_ending, Self::parse_state_ruleset),
                |rulesets| rulesets.into_flattened().into_iter().collect(),
            ),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let ((init_state, diagnostic_steps), rulesets) =
            parser::parse_input_str(input, Self::parse_turing_machine)?;
        Ok(Self {
            init_state,
            diagnostic_steps,
            rulesets,
        })
    }

    fn part1(&self) -> String {
        let mut tape = HashSet::with_hasher(BuildHasherDefault::<FastHasher>::new());
        let mut pos = 0;
        let mut state = self.init_state;
        for _ in 0..self.diagnostic_steps {
            let value = tape.contains(&pos);
            let action = self
                .rulesets
                .get(&Rule { state, value })
                .unwrap_or_else(|| panic!("Not dictated in the blueprint"));
            match (value, action.write) {
                (true, false) => {
                    tape.remove(&pos);
                }
                (false, true) => {
                    tape.insert(pos);
                }
                _ => {}
            }
            pos += if action.move_to_right { 1 } else { -1 };
            state = action.next_state;
        }
        tape.len().to_string()
    }

    fn part2(&self) -> String {
        String::default()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
