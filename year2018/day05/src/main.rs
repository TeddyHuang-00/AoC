use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    polymer: &'static str,
}

impl Puzzle {
    const fn is_paired(left: char, right: char) -> bool {
        (left as u8).abs_diff(right as u8) == 32
    }

    fn react(&self, remove: Option<char>) -> usize {
        self.polymer
            .chars()
            .fold(Vec::with_capacity(self.polymer.len()), |mut stack, ch| {
                if let Some(rem) = remove
                    && (ch == rem || Self::is_paired(ch, rem))
                {
                    return stack;
                }
                if let Some(&last) = stack.last()
                    && Self::is_paired(last, ch)
                {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
                stack
            })
            .len()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &'static str) -> Result<Self> {
        let polymer = input.trim();
        Ok(Self { polymer })
    }

    fn part1(&self) -> String {
        self.react(None).to_string()
    }

    fn part2(&self) -> String {
        ('a'..='z')
            .map(|ch| self.react(Some(ch)))
            .min()
            .unwrap_or_else(|| unreachable!())
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
