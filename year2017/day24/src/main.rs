use anyhow::Result;
use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u8},
    multi::separated_list1,
    sequence::separated_pair,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Default)]
struct EdgeSet(u64);

impl EdgeSet {
    const fn push(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }

    const fn remove(&mut self, idx: usize) {
        self.0 ^= 1 << idx;
    }

    fn pop(&mut self) -> Option<usize> {
        self.0.lowest_one().map(|idx| {
            self.0 &= self.0 - 1;
            idx as usize
        })
    }

    const fn union(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

struct SearchState {
    strength: u16,
    length: u8,
    port: u8,
    edges: EdgeSet,
}

struct Puzzle {
    components: Vec<(u8, u8)>,
    compatible_map: [EdgeSet; 256],
}

impl Puzzle {
    fn parse_component(input: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, char('/'), u8).parse_complete(input)
    }

    fn dfs<F>(&self, mut compare: F)
    where
        F: FnMut(u16, u8),
    {
        let all_edges = EdgeSet((1 << self.components.len()) - 1);
        let mut frontiers = vec![SearchState {
            strength: 0,
            length: 0,
            port: 0,
            edges: all_edges,
        }];
        while let Some(SearchState {
            strength,
            length,
            port,
            edges,
        }) = frontiers.pop()
        {
            compare(strength, length);
            let mut compatibles = edges.union(self.compatible_map[port as usize]);
            while let Some(comp_idx) = compatibles.pop() {
                let mut next_availables = edges;
                next_availables.remove(comp_idx);
                let comp = self.components[comp_idx];
                let next_port = match comp {
                    (x, y) if x == port => y,
                    (x, y) if y == port => x,
                    _ => unreachable!("Should be at least one port matches the current port"),
                };
                frontiers.push(SearchState {
                    strength: strength + u16::from(port) + u16::from(next_port),
                    length: length + 1,
                    port: next_port,
                    edges: next_availables,
                });
            }
        }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let components =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_component))?;
        let mut compatible_map = [EdgeSet::default(); 256];
        for (idx, &(left, right)) in components.iter().enumerate() {
            compatible_map[left as usize].push(idx);
            compatible_map[right as usize].push(idx);
        }
        Ok(Self {
            components,
            compatible_map,
        })
    }

    fn part1(&self) -> String {
        let mut max_strength = 0;
        self.dfs(|strength, _| {
            if strength > max_strength {
                max_strength = strength;
            }
        });
        max_strength.to_string()
    }

    fn part2(&self) -> String {
        let mut best = (0, 0);
        self.dfs(|strength, length| best = best.max((length, strength)));
        best.1.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
