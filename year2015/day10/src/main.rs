use anyhow::Result;
use util::{Solution, chore};

type Uint = u32;

#[derive(Clone, Copy)]
struct Count {
    digit: Uint,
    number: Uint,
}

impl Count {
    const fn inc(&mut self) {
        self.number += 1;
    }

    fn as_digits(count: Self) -> Vec<Uint> {
        count.into()
    }
}

impl From<Uint> for Count {
    fn from(digit: Uint) -> Self {
        Self { digit, number: 1 }
    }
}

impl From<Count> for Vec<Uint> {
    fn from(mut count: Count) -> Self {
        let mut digits = vec![];
        while count.number > 0 {
            digits.push(count.number % 10);
            count.number /= 10;
        }
        digits.reverse();
        digits.push(count.digit);
        digits
    }
}

struct Puzzle {
    counts: Vec<Count>,
}

impl Puzzle {
    fn append_to_counting(mut counts: Vec<Count>, digit: Uint) -> Vec<Count> {
        if let Some(cnt) = counts.last_mut()
            && cnt.digit == digit
        {
            cnt.inc();
        } else {
            counts.push(digit.into());
        }
        counts
    }

    fn look_and_say<A>(counts: A) -> Vec<Count>
    where
        A: AsRef<[Count]>,
    {
        counts
            .as_ref()
            .iter()
            .copied()
            .flat_map(Count::as_digits)
            .fold(vec![], Self::append_to_counting)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let digits = input
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .unwrap_or_else(|| panic!("Invalid input: {c}")) as Uint
            })
            .fold(vec![], Self::append_to_counting);
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
