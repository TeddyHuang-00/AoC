use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, many1_count, separated_list1},
};
use util::{Solution, chore, parser, vector::Vector2D};

#[derive(Clone, Copy)]
enum Direction {
    Up(i8),
    Right(i8),
    Down(i8),
    Left(i8),
}

impl Direction {
    const fn to_vector(self) -> Vector2D<i8> {
        match self {
            Self::Up(y) => Vector2D { x: 0, y: -y },
            Self::Left(x) => Vector2D { x: -x, y: 0 },
            Self::Down(y) => Vector2D { x: 0, y },
            Self::Right(x) => Vector2D { x, y: 0 },
        }
    }
}

const STANDARD_KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
const EXTENDED_KEYPAD: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

struct Puzzle {
    instructions: Vec<Vec<Direction>>,
}

impl Puzzle {
    fn parse_line(input: &str) -> IResult<&str, Vec<Direction>> {
        #[allow(clippy::cast_possible_truncation)]
        many1(alt((
            map(many1_count(char('L')), |cnt| Direction::Left(cnt as i8)),
            map(many1_count(char('R')), |cnt| Direction::Right(cnt as i8)),
            map(many1_count(char('U')), |cnt| Direction::Up(cnt as i8)),
            map(many1_count(char('D')), |cnt| Direction::Down(cnt as i8)),
        )))
        .parse_complete(input)
    }

    fn step(position: &mut Vector2D<u8>, direction: Direction, extended: bool) {
        if extended {
            *position = position.op(direction.to_vector(), |(curr, delta)| {
                curr.saturating_add_signed(delta)
            });
            match direction {
                Direction::Down(_) => position.y = position.y.min(4 - position.x.abs_diff(2)),
                Direction::Up(_) => position.y = position.y.max(position.x.abs_diff(2)),
                Direction::Right(_) => position.x = position.x.min(4 - position.y.abs_diff(2)),
                Direction::Left(_) => position.x = position.x.max(position.y.abs_diff(2)),
            }
        } else {
            *position = position.op(direction.to_vector(), |(curr, delta)| {
                curr.saturating_add_signed(delta).min(2)
            });
        }
    }

    fn position_to_code(position: Vector2D<u8>, extended: bool) -> char {
        if extended {
            EXTENDED_KEYPAD[usize::from(position.y)][usize::from(position.x)]
        } else {
            STANDARD_KEYPAD[usize::from(position.y)][usize::from(position.x)]
        }
    }

    fn get_code(&self, starting_point: Vector2D<u8>, extended: bool) -> String {
        let mut position = starting_point;
        let mut code = Vec::with_capacity(self.instructions.len());
        for instruction in &self.instructions {
            for &direction in instruction {
                Self::step(&mut position, direction, extended);
            }
            code.push(Self::position_to_code(position, extended));
        }
        code.into_iter().collect()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let instructions =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_line))?;
        Ok(Self { instructions })
    }

    fn part1(&self) -> String {
        self.get_code(Vector2D { x: 1, y: 1 }, false)
    }

    fn part2(&self) -> String {
        self.get_code(Vector2D { x: 0, y: 2 }, true)
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
