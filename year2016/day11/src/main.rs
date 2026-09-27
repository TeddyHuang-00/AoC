use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{map, opt, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};
use util::{Solution, chore, integer_as_key, parser};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Component<'a> {
    Generator(&'a str),
    Microchip(&'a str),
}

impl<'a> Component<'a> {
    const fn name(self) -> &'a str {
        match self {
            Self::Generator(s) | Self::Microchip(s) => s,
        }
    }
}

/// First 8 bits are generators, last 8 bits are microchips
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct CompactRepr(u16);

impl CompactRepr {
    const OFFSET: usize = 8;

    const fn generator(self) -> u8 {
        (self.0 >> Self::OFFSET) as u8
    }

    const fn microchip(self) -> u8 {
        ((self.0 << Self::OFFSET) >> Self::OFFSET) as u8
    }

    const fn offset(comp: &Component) -> usize {
        match comp {
            Component::Generator(_) => Self::OFFSET,
            Component::Microchip(_) => 0,
        }
    }

    const fn batch_add(self, comps: Self) -> Self {
        Self(self.0 | comps.0)
    }

    const fn batch_del(self, comps: Self) -> Self {
        Self(self.0 & !comps.0)
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "at most 16 ones, safe for u8"
    )]
    const fn len(self) -> u8 {
        self.0.count_ones() as u8
    }

    fn all_combinations(self) -> Vec<Self> {
        let mut combs = Vec::with_capacity(16);
        let mut all = self.0;

        while all != 0 {
            let low = all.isolate_lowest_one();
            combs.push(Self(low));
            let mut rem = all & (all - 1);

            while rem != 0 {
                let high = rem.isolate_lowest_one();
                combs.push(Self(low | high));

                rem &= rem - 1;
            }

            all &= all - 1;
        }

        combs
    }

    const fn is_empty(self) -> bool {
        self.0 == 0
    }

    const fn is_safe(self) -> bool {
        let g = self.generator();
        let m = self.microchip();
        // Either no generator on the floor,
        // or every chip is protected
        g == 0 || !g & m == 0
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Area {
    floors: [CompactRepr; 4],
    elevator: usize,
}

impl Area {
    const fn is_terminal(self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn possible_moves(self) -> Vec<Self> {
        [-1, 1]
            .into_iter()
            .cartesian_product(self.floors[self.elevator].all_combinations())
            .filter_map(|(movement, carrying)| {
                let elevator = self.elevator.wrapping_add_signed(movement);
                if elevator >= self.floors.len() {
                    return None;
                }
                let prev = self.floors[self.elevator].batch_del(carrying);
                let next = self.floors[elevator].batch_add(carrying);
                if !prev.is_safe() || !next.is_safe() {
                    return None;
                }
                let mut floors = self.floors;
                floors[self.elevator] = prev;
                floors[elevator] = next;
                Some(Self { floors, elevator })
            })
            .collect()
    }
}

#[derive(Clone)]
struct SearchState {
    area: Area,
    cost: u8,
    total: u8,
}

impl SearchState {
    /// The minimum effort of getting things up one floor, the specific order of
    /// taking things doesn't matter
    const fn heuristic(count: u8) -> u8 {
        match count {
            // Nothing to move, or a single stop
            0 | 1 => count,
            // Anything more than 2 would require one extra round trip per item
            2.. => (count - 2) * 2 + 1,
        }
    }
    const fn new(area: Area, cost: u8) -> Self {
        // In best case scenario, we only need to move the first three floors,
        // and only counting the minimum work required. Ignoring all other trips
        // make this optimistic and thus admissible
        let cnt0 = area.floors[0].len();
        let cnt1 = area.floors[1].len();
        let cnt2 = area.floors[2].len();
        let heuristic = Self::heuristic(cnt0)
            + Self::heuristic(cnt0 + cnt1)
            + Self::heuristic(cnt0 + cnt1 + cnt2);
        let total = heuristic + cost;
        Self { area, cost, total }
    }
}

integer_as_key!(SearchState, total);

struct Puzzle {
    initial: Area,
}

impl Puzzle {
    fn parse_floor(input: &str) -> IResult<&str, usize> {
        delimited(
            tag("The "),
            alt((
                value(0, tag("first")),
                value(1, tag("second")),
                value(2, tag("third")),
                value(3, tag("fourth")),
            )),
            tag(" floor"),
        )
        .parse_complete(input)
    }

    fn parse_component(input: &str) -> IResult<&str, Component<'_>> {
        preceded(
            tag("a "),
            alt((
                map(terminated(alpha1, tag(" generator")), Component::Generator),
                map(
                    terminated(alpha1, tag("-compatible microchip")),
                    Component::Microchip,
                ),
            )),
        )
        .parse_complete(input)
    }

    fn parse_layout(input: &str) -> IResult<&str, (usize, Vec<Component<'_>>)> {
        separated_pair(
            Self::parse_floor,
            tag(" contains "),
            terminated(
                alt((
                    separated_list1(
                        alt((preceded(opt(tag(",")), tag(" and ")), tag(", "))),
                        Self::parse_component,
                    ),
                    value(vec![], tag("nothing relevant")),
                )),
                tag("."),
            ),
        )
        .parse_complete(input)
    }

    fn a_star_search(initial: Area) -> u8 {
        let mut visited = HashSet::new();
        let mut frontiers = BinaryHeap::from_iter([Reverse(SearchState::new(initial, 0))]);
        while let Some(Reverse(state)) = frontiers.pop() {
            if state.area.is_terminal() {
                return state.cost;
            }
            frontiers.extend(state.area.possible_moves().into_iter().filter_map(|area| {
                if visited.insert(area) {
                    Some(Reverse(SearchState::new(area, state.cost + 1)))
                } else {
                    None
                }
            }));
        }
        panic!("Cannot find valid solution")
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let mut initial = Area::default();
        let layouts =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_layout))?;
        let names = layouts
            .iter()
            .flat_map(|(_, components)| {
                components.iter().map(|&comp| match comp {
                    Component::Generator(n) | Component::Microchip(n) => n,
                })
            })
            .fold(BTreeMap::new(), |mut acc, name| {
                let idx = acc.len();
                acc.entry(name).or_insert(idx);
                acc
            });
        for (idx, components) in layouts {
            initial.floors[idx] =
                components
                    .into_iter()
                    .fold(CompactRepr::default(), |mut repr, comp| {
                        repr.0 |= 1 << (CompactRepr::offset(&comp) + names[comp.name()]);
                        repr
                    });
        }
        Ok(Self { initial })
    }

    fn part1(&self) -> String {
        Self::a_star_search(self.initial).to_string()
    }

    fn part2(&self) -> String {
        let mut initial = self.initial;
        // We just let the newly added things take the highest two bits to avoid
        // collision, should be safe for the input size.
        let added = 0b1100_0000;
        initial.floors[0] =
            initial.floors[0].batch_add(CompactRepr(added | added << CompactRepr::OFFSET));
        Self::a_star_search(initial).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
