use anyhow::Result;
use util::{Solution, chore};

type Uint = u32;

#[derive(Clone, Copy)]
struct Count {
    digit: u8,
    number: Uint,
}

impl Count {
    const fn inc(&mut self) {
        self.number += 1;
    }
}

impl From<u8> for Count {
    fn from(digit: u8) -> Self {
        Self { digit, number: 1 }
    }
}

struct Puzzle {
    counts: Vec<Count>,
}

impl Puzzle {
    fn append_to_counting(counts: &mut Vec<Count>, digit: u8) {
        if let Some(cnt) = counts.last_mut()
            && cnt.digit == digit
        {
            cnt.inc();
        } else {
            counts.push(digit.into());
        }
    }

    fn look_and_say<A>(counts: A) -> Vec<Count>
    where
        A: AsRef<[Count]>,
    {
        let mut out = Vec::with_capacity(counts.as_ref().len() * 2);

        counts.as_ref().iter().copied().for_each(|mut count| {
            let mut buf = [0; 10];
            let mut idx = 0;
            while count.number > 0 {
                buf[idx] = (count.number % 10) as u8;
                count.number /= 10;
                idx += 1;
            }
            buf[0..idx]
                .iter()
                .rev()
                .for_each(|&x| Self::append_to_counting(&mut out, x));
            Self::append_to_counting(&mut out, count.digit);
        });

        out
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let digits = input
            .trim()
            .chars()
            .map(|c| {
                u8::try_from(
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("Invalid input: {c}")),
                )
                .unwrap_or_else(|_| unreachable!("Single char will never exceed u8"))
            })
            .fold(vec![], |mut acc, ch| {
                Self::append_to_counting(&mut acc, ch);
                acc
            });
        Ok(Self { counts: digits })
    }

    fn part1(&self) -> String {
        (0..40)
            .fold(self.counts.clone(), |counts, _| Self::look_and_say(&counts))
            .into_iter()
            .map(|cnt| cnt.number)
            .sum::<Uint>()
            .to_string()
    }

    fn part2(&self) -> String {
        (0..50)
            .fold(self.counts.clone(), |counts, _| Self::look_and_say(&counts))
            .into_iter()
            .map(|cnt| cnt.number)
            .sum::<Uint>()
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
