use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    sequence::delimited,
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Spell {
    Missile = 53,
    Drain = 73,
    Shield = 113,
    Poison = 173,
    Recharge = 229,
}

impl Spell {
    const fn cost(self) -> u16 {
        self as u16
    }

    const fn min_cost() -> u16 {
        Self::Missile.cost()
    }

    fn valid(mana: u16, effects: &[Effect]) -> impl Iterator<Item = Self> {
        let active = effects
            .iter()
            .map(|effect| match effect {
                Effect::Shield => Self::Shield,
                Effect::Poison => Self::Poison,
                Effect::Recharge => Self::Recharge,
            })
            .collect::<Vec<_>>();
        [
            Self::Missile,
            Self::Drain,
            Self::Shield,
            Self::Poison,
            Self::Recharge,
        ]
        .iter()
        .copied()
        .filter(move |spell| spell.cost() <= mana && !active.contains(spell))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Effect {
    Shield = (6 << 8) + 7,
    Poison = (6 << 8) + 3,
    Recharge = (5 << 8) + 101,
}

impl Effect {
    const fn turns(self) -> u8 {
        ((self as u16) >> 8) as u8
    }

    const fn strength(self) -> u8 {
        ((self as u16) & 0x00ff) as u8
    }
}

#[derive(Clone)]
struct GameState {
    enemy_hp: u8,
    enemy_damage: u8,
    player_hp: u8,
    player_armor: u8,
    mana: u16,
    cost: u16,
    effects: Vec<(Effect, u8)>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for GameState {}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl GameState {
    const fn try_end(&self) -> Option<bool> {
        if self.enemy_hp == 0 {
            return Some(true);
        }
        if self.player_hp == 0 {
            return Some(false);
        }
        None
    }

    fn turn_start(&mut self) {
        self.player_armor = 0;
        for (effect, turns_left) in &mut self.effects {
            match effect {
                Effect::Poison => self.enemy_hp = self.enemy_hp.saturating_sub(effect.strength()),
                Effect::Shield => self.player_armor = effect.strength(),
                Effect::Recharge => self.mana += u16::from(effect.strength()),
            }
            *turns_left -= 1;
        }
        self.effects.retain(|&(_, turns_left)| turns_left > 0);
    }

    fn step(&mut self, spell: Spell, hard_mode: bool) -> Option<bool> {
        // Skip the first turn_start and move it to the end,
        // this way we can get all valid spells
        self.mana -= spell.cost();
        self.cost += spell.cost();
        match spell {
            Spell::Missile => self.enemy_hp = self.enemy_hp.saturating_sub(4),
            Spell::Drain => {
                self.enemy_hp = self.enemy_hp.saturating_sub(2);
                self.player_hp += 2;
            }
            Spell::Shield => self.effects.push((Effect::Shield, Effect::Shield.turns())),
            Spell::Poison => self.effects.push((Effect::Poison, Effect::Poison.turns())),
            Spell::Recharge => self
                .effects
                .push((Effect::Recharge, Effect::Recharge.turns())),
        }
        if let Some(winning) = self.try_end() {
            return Some(winning);
        }

        // Start of the enemy's turn
        self.turn_start();
        if let Some(winning) = self.try_end() {
            return Some(winning);
        }
        self.player_hp = self
            .player_hp
            .saturating_sub(self.enemy_damage.saturating_sub(self.player_armor).max(1));
        if let Some(winning) = self.try_end() {
            return Some(winning);
        }

        // Player's start of turn is moved to here,
        // plus checking if mana is still enough to cast something
        self.turn_start();
        if hard_mode {
            self.player_hp = self.player_hp.saturating_sub(1);
        }
        if let Some(winning) = self.try_end() {
            return Some(winning);
        }
        if self.mana < Spell::min_cost() {
            return Some(false);
        }

        // If hasn't end, just return None to continue
        None
    }
}

struct Puzzle {
    state: GameState,
}

impl Puzzle {
    fn parse_enemy(input: &str) -> IResult<&str, (u8, u8)> {
        (
            delimited(tag("Hit Points: "), u8, line_ending),
            delimited(tag("Damage: "), u8, line_ending),
        )
            .parse_complete(input)
    }

    fn unicost_search(&self, hard_mode: bool) -> u16 {
        let mut best_cost = u16::MAX;
        let initial_state = if hard_mode {
            // Given the order of actual step, we need to reduce hp
            // for the first player round
            let mut state = self.state.clone();
            state.player_hp -= 1;
            state
        } else {
            self.state.clone()
        };
        let mut frontier = BinaryHeap::from([Reverse(initial_state)]);
        while let Some(Reverse(state)) = frontier.pop()
            && state.cost < best_cost
        {
            let effects = state
                .effects
                .iter()
                .map(|(effect, _)| effect)
                .copied()
                .collect::<Vec<_>>();
            for spell in Spell::valid(state.mana, &effects) {
                let mut next = state.clone();
                match next.step(spell, hard_mode) {
                    Some(true) => best_cost = best_cost.min(next.cost),
                    None => frontier.push(Reverse(next)),
                    _ => {}
                }
            }
        }
        best_cost
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (enemy_hp, enemy_damage) = parser::parse_input_str(input, Self::parse_enemy)?;
        let state = GameState {
            enemy_hp,
            enemy_damage,
            player_hp: 50,
            player_armor: 0,
            mana: 500,
            cost: 0,
            effects: vec![],
        };
        Ok(Self { state })
    }

    fn part1(&self) -> String {
        self.unicost_search(false).to_string()
    }

    fn part2(&self) -> String {
        self.unicost_search(true).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
