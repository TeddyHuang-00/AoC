use std::hash::BuildHasherDefault;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending, space0},
    combinator::{map, value},
    multi::{many1, separated_list1},
};
use util::{
    Solution, chore,
    hasher::{FxHashMap, FxHashSet},
    parser,
    vector::Vector2D,
};

type Coord = Vector2D<usize>;
type Velocity = Vector2D<isize>;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn step(self, last: Coord) -> Coord {
        last.op(self.into(), |(v, dv)| v.wrapping_add_signed(dv))
    }
}

impl From<Direction> for Velocity {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self { x: 0, y: -1 },
            Direction::Right => Self { x: 1, y: 0 },
            Direction::Down => Self { x: 0, y: 1 },
            Direction::Left => Self { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
enum Track {
    Vertical,
    Horizontal,
    CurveLLUR,
    CurveULLR,
    Intersection,
}

struct Puzzle {
    turns: FxHashMap<Coord, [Direction; 2]>,
    intersections: FxHashSet<Coord>,
    carts: FxHashMap<Coord, (Direction, u8)>,
}

impl Puzzle {
    fn parse_symbol(input: &str) -> IResult<&str, (Track, Option<Direction>)> {
        alt((
            value((Track::Vertical, None), char('|')),
            value((Track::Horizontal, None), char('-')),
            value((Track::CurveLLUR, None), char('/')),
            value((Track::CurveULLR, None), char('\\')),
            value((Track::Intersection, None), char('+')),
            value((Track::Vertical, Some(Direction::Up)), char('^')),
            value((Track::Horizontal, Some(Direction::Right)), char('>')),
            value((Track::Vertical, Some(Direction::Down)), char('v')),
            value((Track::Horizontal, Some(Direction::Left)), char('<')),
        ))
        .parse_complete(input)
    }

    #[allow(clippy::type_complexity, reason = "Clear enough for me")]
    fn parse_tracks(
        input: &str,
    ) -> IResult<&str, (FxHashMap<Coord, Track>, FxHashMap<Coord, Direction>)> {
        map(
            separated_list1(line_ending, many1((space0, many1(Self::parse_symbol)))),
            |rows| {
                let (coord, items): (Vec<_>, Vec<_>) = rows
                    .into_iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.into_iter()
                            .fold((0, vec![]), |(mut x, mut acc), (space, symbols)| {
                                x += space.len();
                                let s_len = symbols.len();
                                acc.extend(
                                    symbols
                                        .into_iter()
                                        .enumerate()
                                        .map(|(idx, item)| (Vector2D::new(x + idx, y), item)),
                                );
                                (x + s_len, acc)
                            })
                            .1
                    })
                    .unzip();
                let (track, cart): (Vec<_>, Vec<_>) = items.into_iter().unzip();
                let track = coord.iter().copied().zip(track).collect();
                let cart = coord
                    .iter()
                    .copied()
                    .zip(cart)
                    .filter_map(|(c, dir)| dir.map(|d| (c, d)))
                    .collect();
                (track, cart)
            },
        )
        .parse_complete(input)
    }

    fn tick(&self, carts: &mut FxHashMap<Coord, (Direction, u8)>) -> Vec<Coord> {
        let mut update_order = carts.keys().copied().collect::<Vec<_>>();
        update_order.sort_by_key(|coord| (coord.y, coord.x));
        let mut crashes = vec![];
        for mut coord in update_order {
            let Some((mut dir, mut counter)) = carts.remove(&coord) else {
                // This cart has already crashed
                continue;
            };
            coord = coord.op(dir.into(), |(v, dv)| v.wrapping_add_signed(dv));
            if let Some(options) = self.turns.get(&coord) {
                if options.contains(&dir.turn_left()) {
                    dir = dir.turn_left();
                } else {
                    dir = dir.turn_right();
                }
            } else if self.intersections.contains(&coord) {
                match counter {
                    0 => {
                        dir = dir.turn_left();
                    }
                    1 => {}
                    2 => {
                        dir = dir.turn_right();
                    }
                    _ => unreachable!(),
                }
                counter = (counter + 1) % 3;
            }

            if carts.insert(coord, (dir, counter)).is_some() {
                crashes.push(coord);
                carts.remove(&coord);
            }
        }
        crashes
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let (tracks, carts) = parser::parse_input_str(input, Self::parse_tracks)?;
        let (turns, intersections) = tracks.iter().fold(
            (
                FxHashMap::with_hasher(BuildHasherDefault::default()),
                FxHashSet::with_hasher(BuildHasherDefault::default()),
            ),
            |(mut turns, mut intersections), (&coord, &track)| {
                match track {
                    Track::Intersection => {
                        intersections.insert(coord);
                    }
                    Track::CurveLLUR => {
                        if matches!(
                            tracks.get(&Direction::Right.step(coord)),
                            Some(Track::Horizontal | Track::Intersection)
                        ) {
                            turns.insert(coord, [Direction::Right, Direction::Down]);
                        } else {
                            turns.insert(coord, [Direction::Left, Direction::Up]);
                        }
                    }
                    Track::CurveULLR => {
                        if matches!(
                            tracks.get(&Direction::Right.step(coord)),
                            Some(Track::Horizontal | Track::Intersection)
                        ) {
                            turns.insert(coord, [Direction::Right, Direction::Up]);
                        } else {
                            turns.insert(coord, [Direction::Left, Direction::Down]);
                        }
                    }
                    _ => {}
                }
                (turns, intersections)
            },
        );
        let carts = carts
            .into_iter()
            .map(|(coord, direction)| (coord, (direction, 0)))
            .collect();
        Ok(Self {
            turns,
            intersections,
            carts,
        })
    }

    fn part1(&self) -> String {
        let mut carts = self.carts.clone();
        loop {
            let crashes = self.tick(&mut carts);
            if !crashes.is_empty() {
                return format!("{},{}", crashes[0].x, crashes[0].y);
            }
        }
    }

    fn part2(&self) -> String {
        let mut carts = self.carts.clone();
        while carts.len() > 1 {
            self.tick(&mut carts);
        }
        let Some(coord) = carts.into_keys().next() else {
            panic!("No cart left")
        };
        format!("{},{}", coord.x, coord.y)
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
