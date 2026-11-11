use std::collections::{BTreeMap, BTreeSet, VecDeque};

use anyhow::Result;
use ndarray::prelude::*;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{map, value},
    multi::{many1, separated_list1},
};
use util::{Solution, chore, hasher::FxHashSet, parser, vector::Vector2D};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Unit {
    Elf,
    Goblin,
}

type Coord = Vector2D<usize>;
type Map = Array2<bool>;
type UnitCollection = BTreeMap<Coord, (Unit, u8)>;

const HP: u8 = 200;
const DMG: u8 = 3;

const DIRECTIONS: [Vector2D<isize>; 4] = [
    Vector2D { x: -1, y: 0 },
    Vector2D { x: 0, y: -1 },
    Vector2D { x: 0, y: 1 },
    Vector2D { x: 1, y: 0 },
];

#[derive(Clone)]
struct GameState {
    map: Map,
    units: UnitCollection,
    counts: [u8; 2],
    damage: [u8; 2],
}

impl GameState {
    fn remove(&mut self, coord: Coord) -> (Unit, u8) {
        self.map[coord.as_arr()] = false;
        self.units
            .remove(&coord)
            .unwrap_or_else(|| panic!("No such unit at ({},{})", coord.x, coord.y))
    }

    fn insert(&mut self, coord: Coord, status: (Unit, u8)) {
        self.map[coord.as_arr()] = true;
        assert!(
            self.units.insert(coord, status).is_none(),
            "A unit already exists at ({},{})",
            coord.x,
            coord.y
        );
    }

    fn bfs<T, S, F>(&self, origin: Coord, state: &mut T, early_stop: S, call_back: F)
    where
        S: Fn(&T, u8) -> bool,
        F: Fn(&mut T, (Coord, u8), Coord),
    {
        let mut seen = FxHashSet::from_iter([origin]);
        let mut frontier = VecDeque::from([(origin, 0)]);
        while let Some((pos, dist)) = frontier.pop_front() {
            if early_stop(state, dist) {
                break;
            }
            for &dir in &DIRECTIONS {
                let neighbor = pos.op(dir, |(v, dv)| v.wrapping_add_signed(dv));
                if !self.map[neighbor.as_arr()] && !seen.contains(&neighbor) {
                    seen.insert(neighbor);
                    frontier.push_back((neighbor, dist + 1));
                    continue;
                }
                call_back(state, (pos, dist), neighbor);
            }
        }
    }

    fn move_towards_enemy(&mut self, coord: Coord) -> Option<Coord> {
        let unit_type = self.units[&coord].0;
        let mut next_to_enemy = None;
        self.bfs(
            coord,
            &mut next_to_enemy,
            |state, _| state.is_some(),
            |state, (pos, dist), neighbor| {
                if self
                    .units
                    .get(&neighbor)
                    .is_some_and(|&(u, _)| u != unit_type)
                {
                    *state = Some((pos, dist));
                }
            },
        );
        // Found the target cell and its distance
        let (target, distance) = next_to_enemy?;
        // Already next to some enemy
        if distance == 0 {
            return Some(target);
        }
        let mut next_step = BTreeSet::new();
        self.bfs(
            target,
            &mut next_step,
            |_, dist| dist > distance,
            |state, (pos, _), neighbor| {
                if neighbor == coord {
                    state.insert(pos);
                }
            },
        );
        let Some(next) = next_step.pop_first() else {
            unreachable!("Reachable is bi-directional")
        };
        let status = self.remove(coord);
        self.insert(next, status);

        (distance == 1).then_some(target)
    }

    fn act(&mut self, coord: Coord) -> Option<(Unit, Coord)> {
        let attack_position = self.move_towards_enemy(coord)?;
        let (actor_type, _) = self.units[&attack_position];
        // Reading order is handled by DIRECTIONS
        let Some(enemy) = DIRECTIONS
            .iter()
            .copied()
            .filter_map(|dir| {
                let pos = attack_position.op(dir, |(v, dv)| v.wrapping_add_signed(dv));
                self.units
                    .get(&pos)
                    .is_some_and(|&(unit, _)| unit != actor_type)
                    .then_some(pos)
            })
            .min_by_key(|&pos| self.units.get(&pos).unwrap_or_else(|| unreachable!()).1)
        else {
            unreachable!("Must next to an enemy")
        };
        let Some((_, hp)) = self.units.get_mut(&enemy) else {
            unreachable!("Enemy must exist")
        };
        *hp = hp.saturating_sub(self.damage[actor_type as usize]);
        if *hp == 0 {
            self.remove(enemy);
            return Some((
                match actor_type {
                    Unit::Elf => Unit::Goblin,
                    Unit::Goblin => Unit::Elf,
                },
                enemy,
            ));
        }

        None
    }

    fn turn(&mut self) -> bool {
        let mut order = self.units.keys().copied().collect::<VecDeque<_>>();
        while let Some(coord) = order.pop_front() {
            // No more opponents
            if self.counts.contains(&0) {
                return false;
            }
            // Already been defeated
            if !self.units.contains_key(&coord) {
                continue;
            }
            // Action kills an enemy
            if let Some((enemy_type, enemy_coord)) = self.act(coord) {
                self.counts[enemy_type as usize] -= 1;
                if let Some(idx) = order.iter().position(|&c| c == enemy_coord) {
                    // Remove the coordinate from later action queue,
                    // this prevents double action
                    order.remove(idx);
                }
            }
        }

        true
    }
}

struct Puzzle {
    game_state: GameState,
}

impl Puzzle {
    fn parse_cell(input: &str) -> IResult<&str, (bool, Option<Unit>)> {
        alt((
            value((false, None), char('.')),
            value((true, None), char('#')),
            value((true, Some(Unit::Elf)), char('E')),
            value((true, Some(Unit::Goblin)), char('G')),
        ))
        .parse_complete(input)
    }

    fn parse_map(input: &str) -> IResult<&str, (Map, UnitCollection)> {
        map(
            separated_list1(line_ending, many1(Self::parse_cell)),
            |coupled| {
                let nrow = coupled.len();
                let ncol = coupled[0].len();
                let mut map = Array2::from_elem((nrow, ncol), false);
                let mut units = BTreeMap::new();
                for (i, row) in coupled.into_iter().enumerate() {
                    for (j, (occupied, unit)) in row.into_iter().enumerate() {
                        if occupied {
                            map[[i, j]] = occupied;
                        }
                        if let Some(unit) = unit {
                            units.insert(Coord::new(i, j), (unit, HP));
                        }
                    }
                }
                (map, units)
            },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (map, units) = parser::parse_input_str(input, Self::parse_map)?;
        let counts = units.values().copied().fold([0; 2], |mut acc, (u, _)| {
            acc[u as usize] += 1;
            acc
        });
        let game_state = GameState {
            map,
            units,
            counts,
            damage: [DMG; 2],
        };
        Ok(Self { game_state })
    }

    fn part1(&self) -> String {
        let mut state = self.game_state.clone();
        let mut turn_counter = 0;
        while state.turn() {
            turn_counter += 1;
        }
        (state
            .units
            .into_values()
            .map(|(_, hp)| u64::from(hp))
            .sum::<u64>()
            * turn_counter)
            .to_string()
    }

    fn part2(&self) -> String {
        let elf_count = self.game_state.counts[Unit::Elf as usize];
        // We already know this would have elf casualty
        let mut left = 3;
        // We can also safely assume 50 would be more than enough to avoid
        // casualty
        let mut right = 50;
        // Binary search to get the damage we needed
        let mut outcome = 0;
        while left + 1 < right {
            let midpoint = u8::midpoint(left, right);
            let mut state = self.game_state.clone();
            state.damage[Unit::Elf as usize] = midpoint;
            let mut turn_counter = 0;
            let mut success = true;
            while state.turn() {
                turn_counter += 1;
                if state.counts[Unit::Elf as usize] != elf_count {
                    success = false;
                    break;
                }
            }
            success &= state.counts[Unit::Elf as usize] == elf_count;
            if success {
                right = midpoint;
                outcome = state
                    .units
                    .into_values()
                    .map(|(_, hp)| u64::from(hp))
                    .sum::<u64>()
                    * turn_counter;
            } else {
                left = midpoint;
            }
        }
        outcome.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
