use std::collections::BTreeSet;

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u16},
    multi::separated_list1,
    sequence::separated_pair,
};
use util::{Solution, chore, parser};

#[derive(Clone)]
struct DisjointSet(Vec<u16>);

impl DisjointSet {
    fn new(capacity: u16) -> Self {
        Self((0..capacity).collect())
    }

    fn at(&mut self, idx: u16) -> &mut u16 {
        &mut self.0[usize::from(idx)]
    }

    fn find(&mut self, x: u16) -> u16 {
        let mut root = x;
        while root != *self.at(root) {
            root = *self.at(root);
        }
        // Path compression
        let mut ptr = x;
        let mut next;
        while ptr != root {
            next = *self.at(ptr);
            *self.at(ptr) = root;
            ptr = next;
        }
        root
    }

    fn join(&mut self, x: u16, y: u16) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            *self.at(root_y) = *self.at(root_x);
        }
    }
}

struct Puzzle {
    connections: DisjointSet,
    num_nodes: u16,
}

impl Puzzle {
    fn parse_connection(input: &str) -> IResult<&str, (u16, Vec<u16>)> {
        separated_pair(u16, tag(" <-> "), separated_list1(tag(", "), u16)).parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let neighbors =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_connection))?;
        #[allow(
            clippy::cast_possible_truncation,
            reason = "Input as fewer nodes than u16 can hold"
        )]
        let num_nodes = neighbors.len() as u16;
        let mut connections = DisjointSet::new(num_nodes);
        for (x, y) in neighbors.into_iter().flat_map(|(x, ys)| {
            ys.into_iter()
                .filter_map(|y| (x <= y).then_some((x, y)))
                .collect::<Vec<_>>()
        }) {
            connections.join(x, y);
        }
        Ok(Self {
            connections,
            num_nodes,
        })
    }

    fn part1(&self) -> String {
        let mut set = self.connections.clone();
        let root = set.find(0);
        (0..self.num_nodes)
            .filter(|&idx| set.find(idx) == root)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut set = self.connections.clone();
        let mut groups = BTreeSet::new();
        for x in 0..self.num_nodes {
            groups.insert(set.find(x));
        }
        groups.len().to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
