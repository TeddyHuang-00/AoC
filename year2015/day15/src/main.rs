use std::collections::BTreeMap;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, i32, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};
use rayon::prelude::*;
use util::{Solution, chore, parser};

#[derive(Default)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Puzzle {
    ingredients: Vec<Ingredient>,
}

impl Puzzle {
    fn parse_ingredient(input: &str) -> IResult<&str, Ingredient> {
        map(
            (
                alpha1,
                tag(": "),
                preceded(tag("capacity "), i32),
                tag(", "),
                preceded(tag("durability "), i32),
                tag(", "),
                preceded(tag("flavor "), i32),
                tag(", "),
                preceded(tag("texture "), i32),
                tag(", "),
                preceded(tag("calories "), i32),
            ),
            |(_, _, capacity, _, durability, _, flavor, _, texture, _, calories)| Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            },
        )
        .parse_complete(input)
    }

    fn brute_force_all_combinations(&self, calories_limit: Option<i32>) -> i32 {
        (0..self.ingredients.len())
            .combinations_with_replacement(100)
            .par_bridge()
            .filter_map(|choices| {
                let choices = choices.into_iter().fold(BTreeMap::new(), |mut acc, x| {
                    acc.entry(x).and_modify(|cnt| *cnt += 1).or_insert(1);
                    acc
                });
                if choices.len() < self.ingredients.len() {
                    return None;
                }
                let choices = (0..self.ingredients.len())
                    .map(|idx| {
                        *choices
                            .get(&idx)
                            .unwrap_or_else(|| unreachable!("Must be full and valid"))
                    })
                    .collect::<Vec<_>>();
                let sum = self.ingredients.iter().zip(choices.iter()).fold(
                    Ingredient::default(),
                    |acc, (ingredient, count)| Ingredient {
                        capacity: acc.capacity + ingredient.capacity * count,
                        durability: acc.durability + ingredient.durability * count,
                        flavor: acc.flavor + ingredient.flavor * count,
                        texture: acc.texture + ingredient.texture * count,
                        calories: acc.calories + ingredient.calories * count,
                    },
                );
                if sum.capacity <= 0 || sum.durability <= 0 || sum.flavor <= 0 || sum.texture <= 0 {
                    return None;
                }
                if let Some(limit) = calories_limit
                    && sum.calories != limit
                {
                    return None;
                }
                Some(sum.capacity * sum.durability * sum.flavor * sum.texture)
            })
            .max()
            .unwrap_or_else(|| panic!("No positive score found"))
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let ingredients =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_ingredient))?;
        Ok(Self { ingredients })
    }

    fn part1(&self) -> String {
        self.brute_force_all_combinations(None).to_string()
    }

    fn part2(&self) -> String {
        self.brute_force_all_combinations(Some(500)).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
