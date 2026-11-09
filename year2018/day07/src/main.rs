use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
};
use util::{Solution, chore, parser};

struct Puzzle {
    parents: BTreeMap<char, BTreeSet<char>>,
    children: BTreeMap<char, Vec<char>>,
    num_workers: usize,
    task_duration: u8,
}

impl Puzzle {
    fn parse_dependency(input: &str) -> IResult<&str, (char, char)> {
        map(
            terminated(
                separated_pair(
                    preceded(tag("Step "), alpha1),
                    tag(" must be finished before "),
                    preceded(tag("step "), alpha1),
                ),
                tag(" can begin."),
            ),
            |(dep, step): (&str, &str)| {
                (
                    dep.chars().next().unwrap_or_else(|| unreachable!()),
                    step.chars().next().unwrap_or_else(|| unreachable!()),
                )
            },
        )
        .parse_complete(input)
    }

    const fn duration(&self, task: char) -> u8 {
        self.task_duration + task as u8 - b'A' + 1
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let dependency =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_dependency))?;
        let (parents, children) = dependency.into_iter().fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut par, mut chd), (dep, step)| {
                par.entry(step)
                    .and_modify(|set: &mut BTreeSet<_>| {
                        set.insert(dep);
                    })
                    .or_insert_with(|| BTreeSet::from([dep]));
                chd.entry(dep)
                    .and_modify(|lst: &mut Vec<_>| lst.push(step))
                    .or_insert_with(|| vec![step]);
                par.entry(dep).or_insert(BTreeSet::new());
                chd.entry(step).or_insert(vec![]);
                (par, chd)
            },
        );
        let (num_workers, task_duration) = if E { (2, 0) } else { (5, 60) };
        Ok(Self {
            parents,
            children,
            num_workers,
            task_duration,
        })
    }

    fn part1(&self) -> String {
        let mut frontiers = self
            .parents
            .iter()
            .filter_map(|(step, deps)| deps.is_empty().then_some(Reverse(*step)))
            .collect::<BinaryHeap<_>>();
        let mut locks = self.parents.clone();
        let mut resolved = Vec::with_capacity(self.parents.len());
        while let Some(Reverse(step)) = frontiers.pop() {
            resolved.push(step);
            for &child in &self.children[&step] {
                locks.entry(child).and_modify(|set| {
                    set.remove(&step);
                });
                if locks[&child].is_empty() {
                    frontiers.push(Reverse(child));
                }
            }
        }
        resolved.into_iter().collect()
    }

    fn part2(&self) -> String {
        let mut frontiers = self
            .parents
            .iter()
            .filter_map(|(step, deps)| {
                deps.is_empty()
                    .then_some(Reverse((self.duration(*step), *step)))
            })
            .collect::<BinaryHeap<_>>();
        let mut locks = self.parents.clone();
        let mut workers = vec![None; self.num_workers];
        let mut elapsed = 0;
        while let Some(Reverse((dur, step))) = frontiers.pop() {
            let avail_wrkr_idx = workers
                .iter()
                .position(std::option::Option::is_none)
                .unwrap_or_else(|| unreachable!("No worker is available"));
            workers[avail_wrkr_idx] = Some((step, dur));
            // If no more pending jobs, or no more slots available,
            // resolve the worker closest to finishing
            while (frontiers.is_empty() || workers.iter().all(std::option::Option::is_some))
                && let Some((task, time)) = workers
                    .iter()
                    .filter_map(|&w| w)
                    .min_by_key(|&(_, time)| time)
            {
                elapsed += u16::from(time);
                for w in &mut workers {
                    *w = match *w {
                        Some((s, t)) if s != task => Some((s, t - time)),
                        _ => None,
                    }
                }
                for &child in &self.children[&task] {
                    locks.entry(child).and_modify(|set| {
                        set.remove(&task);
                    });
                    if locks[&child].is_empty() {
                        frontiers.push(Reverse((self.duration(child), child)));
                    }
                }
            }
        }
        elapsed.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
