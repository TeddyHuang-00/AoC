use anyhow::Result;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, i32, line_ending, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};
use util::{Solution, chore, parser, vector::Vector2D};

#[derive(Clone, Copy)]
struct Particle {
    position: Vector2D<i32>,
    velocity: Vector2D<i32>,
}

struct Puzzle {
    message: String,
    elapsed: i32,
}

impl Puzzle {
    fn parse_vector(input: &str) -> IResult<&str, Vector2D<i32>> {
        map(
            delimited(
                char('<'),
                separated_pair(preceded(space0, i32), char(','), preceded(space0, i32)),
                char('>'),
            ),
            |(x, y)| Vector2D { x, y },
        )
        .parse_complete(input)
    }

    fn parse_particle(input: &str) -> IResult<&str, Particle> {
        map(
            separated_pair(
                preceded(tag("position="), Self::parse_vector),
                char(' '),
                preceded(tag("velocity="), Self::parse_vector),
            ),
            |(position, velocity)| Particle { position, velocity },
        )
        .parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let particles =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_particle))?;
        let fast_forward = particles
            .iter()
            .copied()
            .map(|Particle { position, velocity }| {
                let t = position.op(
                    -velocity,
                    |(p, v)| {
                        if v == 0 { i32::MAX } else { p / 20 * 20 / v }
                    },
                );
                t.x.min(t.y)
            })
            .min()
            .unwrap_or_else(|| panic!("Particles should not be empty"));
        let mut elapsed = fast_forward;
        let mut message = String::default();
        let (mut width, mut height) = (i32::MAX, i32::MAX);
        for dt in 0..1000 {
            let (x_min, y_min, x_max, y_max) = particles.iter().copied().fold(
                (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
                |(x_min, y_min, x_max, y_max), Particle { position, velocity }| {
                    let pos = position + velocity * (fast_forward + dt);
                    (
                        x_min.min(pos.x),
                        y_min.min(pos.y),
                        x_max.max(pos.x),
                        y_max.max(pos.y),
                    )
                },
            );
            let w = x_max - x_min + 1;
            let h = y_max - y_min + 1;
            if w > width || h > height {
                break;
            }
            width = w;
            height = h;
            if h <= 10 {
                let mut buffer =
                    vec![vec!['.'; w.unsigned_abs() as usize]; h.unsigned_abs() as usize];
                for &Particle { position, velocity } in &particles {
                    let pos = position + velocity * (fast_forward + dt);
                    buffer[pos.y.abs_diff(y_min) as usize][pos.x.abs_diff(x_min) as usize] = '#';
                }
                message = buffer.join(&'\n').into_iter().collect();
                elapsed = fast_forward + dt;
            }
        }
        Ok(Self { message, elapsed })
    }

    fn part1(&self) -> String {
        // Let's just use the more advanced form of computer vision,
        // i.e., human's vision...
        println!("{}", self.message);
        String::default()
    }

    fn part2(&self) -> String {
        self.elapsed.to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
