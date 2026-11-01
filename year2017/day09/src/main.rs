use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{take, take_while1},
    character::complete::char,
    combinator::{map, value},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
};
use util::{Solution, chore, parser};

#[derive(Clone)]
enum Content {
    Group(Box<Group>),
    Garbage(usize),
}

#[derive(Clone)]
struct Group {
    children: Vec<Content>,
}

struct Puzzle {
    group: Group,
}

impl Puzzle {
    fn parse_garbage(input: &str) -> IResult<&str, usize> {
        delimited(
            char('<'),
            map(
                many0(alt((
                    value(0, preceded(char('!'), take(1usize))),
                    map(take_while1(|ch| ch != '!' && ch != '>'), str::len),
                ))),
                |garbages| garbages.into_iter().sum(),
            ),
            char('>'),
        )
        .parse_complete(input)
    }

    fn parse_group(input: &str) -> IResult<&str, Group> {
        map(
            delimited(
                char('{'),
                separated_list0(
                    char(','),
                    alt((
                        map(Self::parse_garbage, Content::Garbage),
                        map(Self::parse_group, |grp| Content::Group(Box::new(grp))),
                    )),
                ),
                char('}'),
            ),
            |children| Group { children },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let group = parser::parse_input_str(input, Self::parse_group)?;
        Ok(Self { group })
    }

    fn part1(&self) -> String {
        let mut frontiers = vec![(1usize, &self.group)];
        let mut sum = 0;
        while let Some((depth, grp)) = frontiers.pop() {
            sum += depth;
            frontiers.extend(grp.children.iter().filter_map(|child| match child {
                Content::Group(g) => Some((depth + 1, g.as_ref())),
                Content::Garbage(_) => None,
            }));
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut frontiers = vec![&self.group];
        let mut sum = 0;
        while let Some(grp) = frontiers.pop() {
            sum += grp
                .children
                .iter()
                .filter_map(|child| match child {
                    Content::Garbage(len) => Some(len),
                    Content::Group(_) => None,
                })
                .sum::<usize>();
            frontiers.extend(grp.children.iter().filter_map(|child| match child {
                Content::Group(g) => Some(g.as_ref()),
                Content::Garbage(_) => None,
            }));
        }
        sum.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
