use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{space1, u8},
    combinator::map,
    multi::many_m_n,
    sequence::{preceded, separated_pair},
};
use util::{Solution, chore, parser};

struct Node {
    children: Vec<Self>,
    metadata: Vec<u8>,
}

impl Node {
    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.metadata.iter().copied().map(u32::from).sum();
        }
        let indices = self
            .metadata
            .iter()
            .copied()
            .fold(BTreeMap::new(), |mut acc, idx| {
                if usize::from(idx) <= self.children.len() {
                    acc.entry(idx - 1).and_modify(|cnt| *cnt += 1).or_insert(1);
                }
                acc
            });
        indices
            .into_iter()
            .map(|(idx, cnt)| cnt * self.children[usize::from(idx)].value())
            .sum()
    }
}

struct Puzzle {
    root: Node,
}

impl Puzzle {
    fn parse_node(input: &str) -> IResult<&str, Node> {
        let (input, (num_children, num_metadata)) =
            separated_pair(u8, space1, u8).parse_complete(input)?;
        map(
            (
                many_m_n(
                    usize::from(num_children),
                    usize::from(num_children),
                    preceded(space1, Self::parse_node),
                ),
                many_m_n(
                    usize::from(num_metadata),
                    usize::from(num_metadata),
                    preceded(space1, u8),
                ),
            ),
            |(children, metadata)| Node { children, metadata },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let root = parser::parse_input_str(input, Self::parse_node)?;
        Ok(Self { root })
    }

    fn part1(&self) -> String {
        let mut stack = vec![&self.root];
        let mut sum = 0;
        while let Some(node) = stack.pop() {
            sum += node.metadata.iter().copied().map(u32::from).sum::<u32>();
            for child in &node.children {
                stack.push(child);
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        self.root.value().to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
