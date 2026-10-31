use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, space1, u16},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};
use util::{Solution, chore, parser};

struct Puzzle {
    names: BTreeMap<u16, &'static str>,
    bottom: BTreeMap<u16, u16>,
    top: BTreeMap<u16, Vec<u16>>,
    weights: BTreeMap<u16, u16>,
}

type Node = (&'static str, u16);
type Children = Vec<&'static str>;

impl Puzzle {
    fn parse_program(input: &'static str) -> IResult<&'static str, (Node, Option<Children>)> {
        (
            separated_pair(alpha1, space1, delimited(char('('), u16, char(')'))),
            opt(preceded(tag(" -> "), separated_list1(tag(", "), alpha1))),
        )
            .parse_complete(input)
    }

    fn root(&self) -> u16 {
        let Some(&root) = self.names.keys().find(|idx| !self.bottom.contains_key(idx)) else {
            panic!("Must be a root in the input")
        };
        root
    }

    fn is_balanced<I>(weights: I) -> bool
    where
        I: Iterator<Item = u16>,
    {
        weights.collect::<BTreeSet<_>>().len() == 1
    }

    fn find_outlier<I>(weights: I) -> (u16, u16)
    where
        I: Iterator<Item = u16>,
    {
        let counts = weights.fold(BTreeMap::new(), |mut acc, t| {
            acc.entry(t).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        });
        assert_eq!(counts.len(), 2, "Must be only two weights");
        (
            *counts
                .iter()
                .max_by_key(|&(_, cnt)| cnt)
                .unwrap_or_else(|| unreachable!())
                .0,
            *counts
                .iter()
                .min_by_key(|&(_, cnt)| cnt)
                .unwrap_or_else(|| unreachable!())
                .0,
        )
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let programs =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_program))?;
        #[allow(
            clippy::cast_possible_truncation,
            reason = "Input range is limited to be within u16"
        )]
        let names = BTreeMap::from_iter(
            programs
                .iter()
                .enumerate()
                .map(|(idx, &((name, _), _))| (name, idx as u16)),
        );
        let weights = BTreeMap::from_iter(
            programs
                .iter()
                .map(|&((name, weight), _)| (names[name], weight)),
        );
        let (bottom, top) = programs.iter().fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut bottom, mut top), ((b, _), ts)| {
                let b = *b;
                if let Some(ts) = ts {
                    for &t in ts {
                        top.entry(names[b])
                            .and_modify(|arr: &mut Vec<_>| arr.push(names[t]))
                            .or_insert_with(|| vec![names[t]]);
                        bottom.insert(names[t], names[b]);
                    }
                }
                (bottom, top)
            },
        );
        let names = names.into_iter().map(|(name, idx)| (idx, name)).collect();

        Ok(Self {
            names,
            bottom,
            top,
            weights,
        })
    }

    fn part1(&self) -> String {
        self.names[&self.root()].to_string()
    }

    fn part2(&self) -> String {
        let mut total_weights = BTreeMap::new();
        for (prog, &weight) in &self.weights {
            let mut prog = Some(prog);
            while let Some(p) = prog {
                total_weights
                    .entry(*p)
                    .and_modify(|w| *w += weight)
                    .or_insert(weight);
                prog = self.bottom.get(p);
            }
        }
        let branches = |idx| self.top[idx].iter().map(|t| total_weights[t]);
        let mut unbalanced = &self.root();
        let mut diff = 0;
        while !Self::is_balanced(branches(unbalanced)) {
            let (normal, outlier) = Self::find_outlier(branches(unbalanced));
            diff = normal
                .checked_signed_diff(outlier)
                .unwrap_or_else(|| panic!("The difference cannot be fit into i16"));
            unbalanced = self.top[unbalanced]
                .iter()
                .find(|&t| total_weights[t] == outlier)
                .unwrap_or_else(|| panic!("Got more than exactly one different weight"));
        }
        self.weights[unbalanced]
            .wrapping_add_signed(diff)
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
