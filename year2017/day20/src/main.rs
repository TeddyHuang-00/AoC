use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::{
        char,
        complete::{isize, line_ending},
    },
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use util::{Solution, chore, parser, vector::Vector3D};

#[derive(Clone, Copy)]
struct Particle {
    position: Vector3D<isize>,
    velocity: Vector3D<isize>,
    acceleration: Vector3D<isize>,
}

#[derive(Clone, Copy, Default)]
struct QuadraticRoots {
    roots: [isize; 2],
    num: u8,
}

impl QuadraticRoots {
    fn push(&mut self, root: isize) {
        self.roots[usize::from(self.num)] = root;
        self.num += 1;
    }
}

impl AsRef<[isize]> for QuadraticRoots {
    fn as_ref(&self) -> &[isize] {
        &self.roots[..usize::from(self.num)]
    }
}

#[derive(Clone, Copy)]
enum AxisRoots {
    All,
    Limited(QuadraticRoots),
}

struct Puzzle {
    particles: Vec<Particle>,
}

impl Puzzle {
    fn parse_vector(input: &str) -> IResult<&str, Vector3D<isize>> {
        map(
            delimited(
                char('<'),
                (isize, char(','), isize, char(','), isize),
                char('>'),
            ),
            |(x, _, y, _, z)| Vector3D::new(x, y, z),
        )
        .parse_complete(input)
    }

    fn parse_particle(input: &str) -> IResult<&str, Particle> {
        map(
            (
                preceded(tag("p="), Self::parse_vector),
                preceded(tag(", v="), Self::parse_vector),
                preceded(tag(", a="), Self::parse_vector),
            ),
            |(p, v, a)| Particle {
                position: p,
                velocity: v,
                acceleration: a,
            },
        )
        .parse_complete(input)
    }

    fn solve_quadratic(a: isize, b: isize, c: isize) -> AxisRoots {
        let mut roots = QuadraticRoots::default();
        // Degrade to linear function
        if a == 0 {
            if b == 0 {
                if c == 0 {
                    return AxisRoots::All;
                }
                return AxisRoots::Limited(roots);
            }
            if c % b == 0 {
                roots.push(-c / b);
            }
            return AxisRoots::Limited(roots);
        }
        let delta = b * b - 4 * a * c;
        // No solution
        if delta < 0 {
            return AxisRoots::Limited(roots);
        }
        let delta_sqrt = delta.isqrt();
        // Solution must not be integer
        if delta_sqrt * delta_sqrt != delta {
            return AxisRoots::Limited(roots);
        }
        // Repeated root
        if delta == 0 {
            if b % (2 * a) == 0 {
                roots.push(-b / (2 * a));
            }
            return AxisRoots::Limited(roots);
        }
        for sign in [-1, 1] {
            if (-b + sign * delta_sqrt) % (2 * a) == 0 {
                roots.push((-b + sign * delta_sqrt) / (2 * a));
            }
        }
        AxisRoots::Limited(roots)
    }

    fn solve_quadratic_system(
        a: Vector3D<isize>,
        b: Vector3D<isize>,
        c: Vector3D<isize>,
    ) -> QuadraticRoots {
        let roots = [(a.x, b.x, c.x), (a.y, b.y, c.y), (a.z, b.z, c.z)]
            .into_iter()
            .map(|(a, b, c)| Self::solve_quadratic(a, b, c))
            .reduce(|left, right| {
                match (left, right) {
                    (AxisRoots::All, roots) | (roots, AxisRoots::All) => roots,
                    (AxisRoots::Limited(left), AxisRoots::Limited(right)) => {
                        let mut roots = QuadraticRoots::default();
                        for &t in left.as_ref() {
                            // We only keep non-negative roots
                            if t >= 0 && right.as_ref().contains(&t) {
                                roots.push(t);
                            }
                        }
                        AxisRoots::Limited(roots)
                    }
                }
            })
            .unwrap_or_else(|| unreachable!("Must be exactly three elements"));
        match roots {
            AxisRoots::All => QuadraticRoots {
                roots: [0, 0],
                num: 1,
            },
            AxisRoots::Limited(roots) => roots,
        }
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let particles =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_particle))?;
        Ok(Self { particles })
    }

    fn part1(&self) -> String {
        self.particles
            .iter()
            .enumerate()
            .min_by_key(
                |(
                    _,
                    Particle {
                        position: _,
                        velocity: _,
                        acceleration,
                    },
                )| {
                    acceleration.x.abs() + acceleration.y.abs() + acceleration.z.abs()
                },
            )
            .unwrap_or_else(|| panic!("List should not be empty"))
            .0
            .to_string()
    }

    fn part2(&self) -> String {
        let collisions = (0..self.particles.len())
            .array_combinations()
            .filter_map(|[left_idx, right_idx]| {
                let left = self.particles[left_idx];
                let right = self.particles[right_idx];
                let a = left.acceleration - right.acceleration;
                let b = (left.velocity - right.velocity) * 2 + a;
                let c = (left.position - right.position) * 2;
                let roots = Self::solve_quadratic_system(a, b, c);
                (roots.num > 0).then_some((roots, (left_idx, right_idx)))
            })
            .fold(BTreeMap::new(), |mut acc, (root, pair)| {
                for &r in root.as_ref() {
                    acc.entry(r)
                        .and_modify(|lst: &mut Vec<_>| lst.push(pair))
                        .or_insert_with(|| vec![pair]);
                }
                acc
            });
        let mut destroyed = BTreeSet::new();
        for (_, pairs) in collisions {
            let to_be_destroyed = pairs
                .into_iter()
                .filter_map(|(left, right)| {
                    (!destroyed.contains(&left) && !destroyed.contains(&right))
                        .then_some([left, right])
                })
                .flatten()
                .collect::<BTreeSet<_>>();
            destroyed.extend(to_be_destroyed);
        }
        (self.particles.len() - destroyed.len()).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
