use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    combinator::map,
    sequence::delimited,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
struct Item {
    cost: u8,
    damage: u8,
    armor: u8,
}

impl Item {
    const fn new(cost: u8, damage: u8, armor: u8) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMOR: [Item; 5] = [
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

struct Character {
    hp: u8,
    damage: u8,
    armor: u8,
}

impl Character {
    fn will_defeat(&self, other: &Self) -> bool {
        let attack_turns = other
            .hp
            .div_ceil(self.damage.saturating_sub(other.armor).max(1));
        let defend_turns = self
            .hp
            .div_ceil(other.damage.saturating_sub(self.armor).max(1));
        attack_turns <= defend_turns
    }
}

struct Puzzle {
    enemy: Character,
}

impl Puzzle {
    fn parse_enemy(input: &str) -> IResult<&str, Character> {
        map(
            (
                delimited(tag("Hit Points: "), u8, line_ending),
                delimited(tag("Damage: "), u8, line_ending),
                delimited(tag("Armor: "), u8, line_ending),
            ),
            |(hp, damage, armor)| Character { hp, damage, armor },
        )
        .parse_complete(input)
    }

    fn iterate_all_sets() -> impl Iterator<Item = (Item, (Vec<Item>, Vec<Item>))> {
        WEAPONS.iter().copied().cartesian_product(
            ARMOR
                .iter()
                .copied()
                .combinations(0)
                .chain(ARMOR.iter().copied().combinations(1))
                .cartesian_product(
                    RINGS
                        .iter()
                        .copied()
                        .combinations(0)
                        .chain(RINGS.iter().copied().combinations(1))
                        .chain(RINGS.iter().copied().combinations(2)),
                ),
        )
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let enemy = parser::parse_input_str(input, Self::parse_enemy)?;
        Ok(Self { enemy })
    }

    fn part1(&self) -> String {
        Self::iterate_all_sets()
            .par_bridge()
            .filter_map(|(weapon, (armor, ring))| {
                let mut cost = u16::from(weapon.cost);
                let mut player = Character {
                    hp: 100,
                    damage: weapon.damage,
                    armor: weapon.armor,
                };
                for item in armor.into_iter().chain(ring) {
                    cost += u16::from(item.cost);
                    player.damage += item.damage;
                    player.armor += item.armor;
                }
                player.will_defeat(&self.enemy).then_some(cost)
            })
            .min()
            .unwrap_or_else(|| panic!("No way to defeat the enemy"))
            .to_string()
    }

    fn part2(&self) -> String {
        Self::iterate_all_sets()
            .par_bridge()
            .filter_map(|(weapon, (armor, ring))| {
                let mut cost = u16::from(weapon.cost);
                let mut player = Character {
                    hp: 100,
                    damage: weapon.damage,
                    armor: weapon.armor,
                };
                for item in armor.into_iter().chain(ring) {
                    cost += u16::from(item.cost);
                    player.damage += item.damage;
                    player.armor += item.armor;
                }
                (!player.will_defeat(&self.enemy)).then_some(cost)
            })
            .max()
            .unwrap_or_else(|| panic!("No way to lost the fight"))
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
