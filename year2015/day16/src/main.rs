use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u8, u16},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Read {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

#[derive(Clone, Copy, Default)]
struct Aunt([Option<u8>; 10]);

impl From<Vec<(Read, u8)>> for Aunt {
    fn from(value: Vec<(Read, u8)>) -> Self {
        value
            .into_iter()
            .fold(Self::default(), |mut aunt, (name, count)| {
                aunt.0[name as usize] = Some(count);
                aunt
            })
    }
}

impl PartialEq for Aunt {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .into_iter()
            .zip(other.0)
            .all(|(left, right)| left.is_none_or(|lv| right.is_none_or(|rv| lv == rv)))
    }
}

impl PartialOrd for Aunt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Read::{
            Akitas, Cars, Cats, Children, Goldfish, Perfumes, Pomeranians, Samoyeds, Trees, Vizslas,
        };
        [
            Children,
            Cats,
            Samoyeds,
            Pomeranians,
            Akitas,
            Vizslas,
            Goldfish,
            Trees,
            Cars,
            Perfumes,
        ]
        .into_iter()
        .all(|idx| {
            self.0[idx as usize].is_none_or(|lv| {
                other.0[idx as usize].is_none_or(|rv| match idx {
                    Cats | Trees => lv > rv,
                    Pomeranians | Goldfish => lv < rv,
                    _ => lv == rv,
                })
            })
        })
        .then_some(std::cmp::Ordering::Less)
        .or(Some(std::cmp::Ordering::Greater))
    }
}

struct Puzzle {
    aunts: Vec<(u16, Aunt)>,
    reference: Aunt,
}

impl Puzzle {
    fn parse_aunt(input: &str) -> IResult<&str, (u16, Aunt)> {
        use Read::{
            Akitas, Cars, Cats, Children, Goldfish, Perfumes, Pomeranians, Samoyeds, Trees, Vizslas,
        };
        map(
            (
                delimited(tag("Sue "), u16, tag(": ")),
                separated_list1(
                    tag(", "),
                    separated_pair(
                        alt((
                            value(Children, tag("children")),
                            value(Cats, tag("cats")),
                            value(Samoyeds, tag("samoyeds")),
                            value(Pomeranians, tag("pomeranians")),
                            value(Akitas, tag("akitas")),
                            value(Vizslas, tag("vizslas")),
                            value(Goldfish, tag("goldfish")),
                            value(Trees, tag("trees")),
                            value(Cars, tag("cars")),
                            value(Perfumes, tag("perfumes")),
                        )),
                        tag(": "),
                        u8,
                    ),
                ),
            ),
            |(idx, items)| (idx, items.into()),
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        use Read::{
            Akitas, Cars, Cats, Children, Goldfish, Perfumes, Pomeranians, Samoyeds, Trees, Vizslas,
        };
        let aunts = parser::parse_input_str(input, separated_list1(line_ending, Self::parse_aunt))?;
        let reference = vec![
            (Children, 3),
            (Cats, 7),
            (Samoyeds, 2),
            (Pomeranians, 3),
            (Akitas, 0),
            (Vizslas, 0),
            (Goldfish, 5),
            (Trees, 3),
            (Cars, 2),
            (Perfumes, 1),
        ]
        .into();
        Ok(Self { aunts, reference })
    }

    fn part1(&self) -> String {
        let candidates = self
            .aunts
            .iter()
            .filter_map(|&(idx, aunt)| (aunt == self.reference).then_some(idx))
            .collect::<Vec<_>>();
        assert!(
            candidates.len() == 1,
            "Expect exactly 1 aunt, found: {}",
            candidates.len()
        );
        candidates[0].to_string()
    }

    fn part2(&self) -> String {
        let candidates = self
            .aunts
            .iter()
            .filter_map(|&(idx, aunt)| (aunt < self.reference).then_some(idx))
            .collect::<Vec<_>>();
        assert!(
            candidates.len() == 1,
            "Expect exactly 1 aunt, found: {}",
            candidates.len()
        );
        candidates[0].to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
