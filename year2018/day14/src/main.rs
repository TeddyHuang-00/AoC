use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    index: usize,
}

impl Puzzle {
    fn create_recipes(recipes: &mut Vec<u8>, elf1: &mut usize, elf2: &mut usize) {
        let sum = recipes[*elf1] + recipes[*elf2];
        if sum >= 10 {
            recipes.extend([sum / 10, sum % 10]);
        } else {
            recipes.push(sum % 10);
        }
        *elf1 += 1 + usize::from(recipes[*elf1]);
        *elf1 %= recipes.len();
        *elf2 += 1 + usize::from(recipes[*elf2]);
        *elf2 %= recipes.len();
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let index = input.trim().parse()?;
        Ok(Self { index })
    }

    fn part1(&self) -> String {
        let mut recipes = Vec::with_capacity(self.index + 20);
        recipes.extend([3, 7]);
        let (mut elf1, mut elf2) = (0, 1);
        while recipes.len() < self.index + 10 {
            Self::create_recipes(&mut recipes, &mut elf1, &mut elf2);
        }
        recipes[self.index..self.index + 10]
            .iter()
            .map(|&digit| (b'0' + digit) as char)
            .collect()
    }

    fn part2(&self) -> String {
        let mut pattern = (0..10)
            .scan(self.index, |val, _| {
                (*val > 0).then(|| {
                    #[allow(
                        clippy::cast_possible_truncation,
                        reason = "Single digit convert to u8"
                    )]
                    let v = (*val % 10) as u8;
                    *val /= 10;
                    v
                })
            })
            .collect::<Vec<_>>();
        pattern.reverse();
        let mut recipes = Vec::with_capacity(100_000_000);
        recipes.extend([3, 7]);
        let (mut elf1, mut elf2) = (0, 1);
        let mut index = 0;
        loop {
            for _ in 0..1_000 {
                Self::create_recipes(&mut recipes, &mut elf1, &mut elf2);
            }
            if let Some(offset) = recipes[index..]
                .windows(pattern.len())
                .enumerate()
                .find_map(|(idx, seq)| (seq == &pattern[..]).then_some(idx))
            {
                return (index + offset).to_string();
            }
            index = recipes.len() - pattern.len() + 1;
        }
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
