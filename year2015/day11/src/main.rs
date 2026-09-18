use std::collections::BTreeSet;

use anyhow::Result;
use util::{Solution, chore};

#[derive(Clone, Copy)]
struct Password([u8; 8]);

impl Password {
    fn inc(&mut self) -> Self {
        let mut carry = true;
        let mut iter = self.0.iter_mut().rev();
        while carry {
            if let Some(ch) = iter.next() {
                let x = *ch + 1;
                if x > b'z' {
                    *ch = b'a';
                } else {
                    *ch = x;
                    carry = false;
                }
            }
        }
        *self
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str::from_utf8(&self.0).map_err(|_| std::fmt::Error)?)
    }
}

struct Puzzle {
    seed: Password,
}

impl Puzzle {
    fn is_valid(password: Password) -> bool {
        password
            .0
            .array_windows::<3>()
            .any(|&[a, b, c]| a + 1 == b && b + 1 == c)
            && !password.0.iter().any(|ch| b"iol".contains(ch))
            && password
                .0
                .array_windows::<2>()
                .filter_map(|&[a, b]| (a == b).then_some(a))
                .collect::<BTreeSet<_>>()
                .len()
                >= 2
    }

    fn find_next_valid(mut password: Password) -> Password {
        loop {
            password.inc();
            if Self::is_valid(password) {
                return password;
            }
        }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let seed = Password(
            input
                .trim()
                .char_indices()
                .inspect(|&(idx, _)| {
                    assert!(
                        idx < 8,
                        "Input should not be longer than 8 chars: {}",
                        input.trim()
                    );
                })
                .fold([0; 8], |mut arr, (idx, ch)| {
                    arr[idx] = ch as u8;
                    arr
                }),
        );
        Ok(Self { seed })
    }

    fn part1(&self) -> String {
        Self::find_next_valid(self.seed).to_string()
    }

    fn part2(&self) -> String {
        let last = Self::find_next_valid(self.seed);
        Self::find_next_valid(last).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
