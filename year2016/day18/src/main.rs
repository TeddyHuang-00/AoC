use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::{many1, many1_count},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy)]
enum Tile {
    Safe(u8),
    Trap(u8),
}

// Trap is 1, Safe is 0, so that default padding is safe
type Row = u128;

struct Puzzle {
    init: Row,
    ncol: usize,
    nrows: (usize, usize),
}

impl Puzzle {
    #[allow(clippy::cast_possible_truncation)]
    fn parse_tiles(input: &str) -> IResult<&str, Vec<Tile>> {
        many1(alt((
            map(many1_count(char('.')), |cnt| Tile::Safe(cnt as u8)),
            map(many1_count(char('^')), |cnt| Tile::Trap(cnt as u8)),
        )))
        .parse_complete(input)
    }

    fn count_safe_till_row(&self, nrow: usize) -> u32 {
        let mask = !(u128::MAX << self.ncol);
        let mut row = self.init;
        let mut safe = (row | !mask).count_zeros();
        for _ in 1..nrow {
            row = (row << 1 ^ row >> 1) & mask;
            safe += (row | !mask).count_zeros();
        }
        safe
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let tiles = parser::parse_input_str(input, Self::parse_tiles)?;
        let ncol = tiles
            .iter()
            .map(|&tile| match tile {
                Tile::Safe(cnt) | Tile::Trap(cnt) => cnt as usize,
            })
            .sum();
        let init = tiles.into_iter().fold(0, |acc, tile| match tile {
            Tile::Safe(cnt) => acc << cnt,
            Tile::Trap(cnt) => acc << cnt | !(u128::MAX << cnt),
        });
        let nrows = if E { (10, 10) } else { (40, 400_000) };
        Ok(Self { init, ncol, nrows })
    }

    fn part1(&self) -> String {
        self.count_safe_till_row(self.nrows.0).to_string()
    }

    fn part2(&self) -> String {
        self.count_safe_till_row(self.nrows.1).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
