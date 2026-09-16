use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

type Signal = u16;
type StaticStr = &'static str;

#[derive(Clone, Debug)]
enum Source {
    Wire(StaticStr),
    Value(Signal),
}

#[derive(Clone, Debug)]
enum Transform {
    Identity(Source),
    Not(Source),
    And((Source, Source)),
    Or((Source, Source)),
    LShift((Source, Signal)),
    RShift((Source, Signal)),
}

trait Node {
    fn as_parents(&self) -> Vec<StaticStr>;
}

impl Node for Signal {
    fn as_parents(&self) -> Vec<StaticStr> {
        vec![]
    }
}

impl<T, U> Node for (T, U)
where
    T: Node,
    U: Node,
{
    fn as_parents(&self) -> Vec<StaticStr> {
        let mut parents = Vec::with_capacity(2);
        parents.extend(self.0.as_parents());
        parents.extend(self.1.as_parents());
        parents
    }
}

impl Node for Source {
    fn as_parents(&self) -> Vec<StaticStr> {
        match self {
            Self::Wire(wire) => vec![wire],
            Self::Value(_) => vec![],
        }
    }
}

macro_rules! repeat_match_arms {
    ($self:ident, $fn:ident, $($lst:ident),+) => {
        match $self {
            $($lst(x) => x.$fn(),)+
        }
    };
}

impl Node for Transform {
    fn as_parents(&self) -> Vec<StaticStr> {
        use Transform::{And, Identity, LShift, Not, Or, RShift};
        repeat_match_arms!(self, as_parents, Identity, Not, And, Or, LShift, RShift)
    }
}

struct Puzzle {
    wires: BTreeSet<StaticStr>,
    input: BTreeMap<StaticStr, Transform>,
    output: BTreeMap<StaticStr, Vec<StaticStr>>,
}

impl Puzzle {
    fn parse_source(input: StaticStr) -> IResult<StaticStr, Source> {
        use nom::character::complete::u16;
        alt((
            map(u16, Source::Value),
            map(alpha1, |s: StaticStr| Source::Wire(s)),
        ))
        .parse_complete(input)
    }

    fn parse_transform(input: StaticStr) -> IResult<StaticStr, Transform> {
        use nom::character::complete::u16;
        alt((
            map(preceded(tag("NOT "), Self::parse_source), Transform::Not),
            map(
                separated_pair(Self::parse_source, tag(" AND "), Self::parse_source),
                Transform::And,
            ),
            map(
                separated_pair(Self::parse_source, tag(" OR "), Self::parse_source),
                Transform::Or,
            ),
            map(
                separated_pair(Self::parse_source, tag(" LSHIFT "), u16),
                Transform::LShift,
            ),
            map(
                separated_pair(Self::parse_source, tag(" RSHIFT "), u16),
                Transform::RShift,
            ),
            map(Self::parse_source, Transform::Identity),
        ))
        .parse_complete(input)
    }

    fn parse_wire(input: StaticStr) -> IResult<StaticStr, (Transform, StaticStr)> {
        separated_pair(Self::parse_transform, tag(" -> "), alpha1)
            .map(|(transform, wire)| (transform, wire))
            .parse_complete(input)
    }

    fn evaluate_source(src: &Source, signals: &BTreeMap<&str, Signal>) -> Signal {
        let Some(signal) = (match src {
            Source::Wire(wire) => signals.get(wire).copied(),
            Source::Value(val) => Some(*val),
        }) else {
            panic!("Source {src:?} has no signal value in the current context: {signals:?}")
        };
        signal
    }

    fn evaluate_transform(trans: &Transform, signals: &BTreeMap<&str, Signal>) -> Signal {
        match trans {
            Transform::Identity(src) => Self::evaluate_source(src, signals),
            Transform::Not(src) => !Self::evaluate_source(src, signals),
            Transform::And((src1, src2)) => {
                Self::evaluate_source(src1, signals) & Self::evaluate_source(src2, signals)
            }
            Transform::Or((src1, src2)) => {
                Self::evaluate_source(src1, signals) | Self::evaluate_source(src2, signals)
            }
            Transform::LShift((src, shift)) => Self::evaluate_source(src, signals) << shift,
            Transform::RShift((src, shift)) => Self::evaluate_source(src, signals) >> shift,
        }
    }

    fn emulate(&self, custom_input: Option<&BTreeMap<&str, Transform>>) -> Signal {
        let input = custom_input.unwrap_or(&self.input);
        let mut parents = input
            .iter()
            .map(|(&wire, trans)| (wire, trans.as_parents().len()))
            .collect::<BTreeMap<_, _>>();
        let mut signals = self
            .wires
            .iter()
            .map(|&wire| (wire, 0))
            .collect::<BTreeMap<_, _>>();
        let mut frontiers = parents
            .iter()
            .filter_map(|(&wire, &to_be_determined)| (to_be_determined == 0).then_some(wire))
            .collect::<Vec<_>>();

        while let Some(wire) = frontiers.pop() {
            // Evaluate the signal for this wire based on its transform and the
            // signals of its parents
            let Some(input) = input.get(wire) else {
                unreachable!("Wire must have an input")
            };
            let signal = Self::evaluate_transform(input, &signals);
            signals.insert(wire, signal);
            // Update the parents count for the children of this wire
            if let Some(children) = self.output.get(wire) {
                for child in children {
                    let Some(count) = parents.get_mut(child) else {
                        unreachable!("A child must have a parent")
                    };
                    *count -= 1;
                    if *count == 0 {
                        frontiers.push(child);
                    }
                }
            }
        }

        // Return the signal for wire "a"
        signals
            .get("a")
            .copied()
            .unwrap_or_else(|| panic!("a is not available"))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: StaticStr) -> Result<Self> {
        let wiring =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_wire))?;
        let wires = wiring.iter().map(|(_, id)| *id).collect();
        let input = wiring
            .iter()
            .map(|(trans, id)| (*id, trans.clone()))
            .collect();
        let output = wiring
            .into_iter()
            .fold(BTreeMap::new(), |mut out, (trans, id)| {
                for parent in trans.as_parents() {
                    out.entry(parent).or_insert(vec![]).push(id);
                }
                out
            });
        Ok(Self {
            wires,
            input,
            output,
        })
    }

    fn part1(&self) -> String {
        self.emulate(None).to_string()
    }

    fn part2(&self) -> String {
        // Run first pass to get the signal for wire "a".
        // With almost no effort, this computation can be skipped,
        // but I will leave it here to make each part self-contained.
        let signal_a = self.emulate(None);
        // Override the input for wire "b" with the signal from wire "a"
        let mut custom_input = self.input.clone();
        custom_input.insert("b", Transform::Identity(Source::Value(signal_a)));
        self.emulate(Some(&custom_input)).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
