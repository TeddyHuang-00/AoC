use std::{collections::BTreeMap, ops::Range};

use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

const MAX_N: usize = 8;

#[derive(Clone, Copy)]
struct NodePack(u8);

impl<A> From<A> for NodePack
where
    A: AsRef<[u8]>,
{
    fn from(value: A) -> Self {
        Self(value.as_ref().iter().fold(0u8, |acc, v| acc | 1 << v))
    }
}

impl From<NodePack> for Vec<u8> {
    fn from(value: NodePack) -> Self {
        let mut nodes = Self::new();
        let mut value = value.0;
        while value.leading_zeros() < 8 {
            #[allow(clippy::cast_possible_truncation)]
            let idx = 7 - value.leading_zeros() as u8;
            nodes.push(idx);
            value &= !(1 << idx);
        }
        nodes
    }
}

impl From<NodePack> for usize {
    fn from(value: NodePack) -> Self {
        Self::from(value.0)
    }
}

impl NodePack {
    const fn remove(mut self, node: u8) -> Self {
        self.0 &= !(1 << node);
        self
    }

    const fn as_index(self) -> usize {
        self.0 as usize
    }

    fn as_vec(self) -> Vec<u8> {
        self.into()
    }
}

macro_rules! np {
    ($($x:expr),*) => {
        #[allow(clippy::cast_possible_truncation)]
        NodePack::from([$($x as u8),*])
    }
}

const fn dp_index(node_pack: NodePack, start: u8) -> usize {
    (node_pack.as_index() << 3) + start as usize
}

const fn dp_range(node_pack: NodePack, start: u8, end: u8) -> Range<usize> {
    Range {
        start: dp_index(node_pack, start),
        end: dp_index(node_pack, end),
    }
}

struct Puzzle {
    distances: Array2<u16>,
}

impl Puzzle {
    fn parse_pair(input: &str) -> IResult<&str, ((&str, &str), u16)> {
        use nom::character::complete::u16;
        separated_pair(separated_pair(alpha1, tag(" to "), alpha1), tag(" = "), u16)
            .parse_complete(input)
    }

    fn tsp_dp(&self, shortest: bool) -> u16 {
        let (init, cmp): (u16, fn(u16, u16) -> u16) = if shortest {
            (u16::MAX, u16::min)
        } else {
            (u16::MIN, u16::max)
        };
        let mut dp = [init; MAX_N << 8];
        let n = self.distances.ncols();
        // Initialize the distances for pairs of nodes
        #[allow(clippy::cast_possible_truncation)]
        for pair in (0..n).combinations(2) {
            let (i, j) = (pair[0], pair[1]);
            dp[dp_index(np![i, j], i as u8)] = self.distances[[i, j]];
            dp[dp_index(np![i, j], j as u8)] = self.distances[[j, i]];
        }
        // Main DP loop
        for size in 3..=n {
            // Find out the shortest traversal of all subgraphs of this size
            #[allow(clippy::cast_possible_truncation)]
            for nodes in (0..n as u8).combinations(size) {
                let set = NodePack::from(&nodes);
                let min_dist = nodes
                    .par_iter()
                    .copied()
                    .flat_map(|start| {
                        // Break down the problem into starting from any node in
                        // the subgraph, and find the
                        // shortest by trying first going to another node and
                        // traverse the rest of the subgraph (smaller, must be
                        // known) from there
                        let rest = set.remove(start);
                        rest.as_vec()
                            .into_iter()
                            .map(|next| (start, next, rest))
                            .collect::<Vec<_>>()
                    })
                    .fold(
                        || [init; MAX_N],
                        |mut dist, (start, next, rest)| {
                            dist[usize::from(start)] = cmp(
                                dist[usize::from(start)],
                                // First go from start to next, then from next to traversal of rest
                                dp[dp_index(np![start, next], start)] + dp[dp_index(rest, next)],
                            );
                            dist
                        },
                    )
                    .reduce(
                        || [init; MAX_N],
                        |mut dist1, dist2| {
                            dist1
                                .iter_mut()
                                .zip(dist2)
                                .for_each(|(d1, d2)| *d1 = cmp(*d1, d2));
                            dist1
                        },
                    );

                // Update the dp to store the results
                set.as_vec()
                    .into_iter()
                    .for_each(|start| dp[dp_index(set, start)] = min_dist[usize::from(start)]);
            }
        }
        #[allow(clippy::cast_possible_truncation)]
        dp[dp_range(NodePack::from((0..n as u8).collect::<Vec<_>>()), 0, n as u8)]
            .iter()
            .copied()
            .reduce(cmp)
            .unwrap_or_else(|| unreachable!("given range must not be empty"))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let paired_distances: Vec<((&'static str, &'static str), u16)> =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_pair))?;
        let node_idx = paired_distances
            .iter()
            .flat_map(|pair| [pair.0.0, pair.0.1])
            .fold(BTreeMap::new(), |mut acc, x| {
                let len = acc.len();
                acc.entry(x).or_insert_with(|| len);
                acc
            });
        let n = node_idx.len();
        assert!(n <= MAX_N, "The puzzle input is relatively small in size");
        let paired_distances = paired_distances
            .into_iter()
            .map(|(pair, distance)| {
                let idx1 = *node_idx
                    .get(pair.0)
                    .unwrap_or_else(|| unreachable!("Node must be in the list"));
                let idx2 = *node_idx
                    .get(pair.1)
                    .unwrap_or_else(|| unreachable!("Node must be in the list"));
                ((idx1.min(idx2), idx1.max(idx2)), distance)
            })
            .collect::<BTreeMap<_, _>>();
        let distances = Array2::from_shape_fn((n, n), |(i, j)| {
            *paired_distances.get(&(i.min(j), i.max(j))).unwrap_or(&0) // Node distance to self is 0
        });
        Ok(Self { distances })
    }

    fn part1(&self) -> String {
        self.tsp_dp(true).to_string()
    }

    fn part2(&self) -> String {
        self.tsp_dp(false).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
