use std::mem::swap;

use anyhow::Result;
use ndarray::{Zip, parallel::prelude::*, prelude::*};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use util::{Solution, chore, parser};

struct Pattern(Vec<u16>);

impl Pattern {
    fn hash(grid: &ArrayView2<bool>) -> u16 {
        grid.iter().fold(0, |mut acc, &elem| {
            acc <<= 1;
            acc |= u16::from(elem);
            acc
        })
    }
}

struct Transformation {
    pattern: Pattern,
    replace: Array2<bool>,
}

struct RuleSet {
    replacements: Vec<Array2<bool>>,
    pattern_mapping: Vec<usize>,
}

impl RuleSet {
    fn new(transformations: Vec<Transformation>) -> Self {
        let mut replacements = Vec::with_capacity(transformations.len());
        let size = transformations[0].replace.ncols() - 1;
        let mut pattern_mapping = vec![0; 1 << (size * size)];
        for trans in transformations {
            for mat in trans.pattern.0 {
                pattern_mapping[usize::from(mat)] = replacements.len();
            }
            replacements.push(trans.replace);
        }
        Self {
            replacements,
            pattern_mapping,
        }
    }

    fn transform(&self, hash: u16) -> ArrayView2<'_, bool> {
        self.replacements[self.pattern_mapping[usize::from(hash)]].view()
    }
}

struct Puzzle {
    trans_2_to_3: RuleSet,
    trans_3_to_4: RuleSet,
    seed: Array2<bool>,
}

impl Puzzle {
    fn parse_grid(input: &str) -> IResult<&str, Array2<bool>> {
        map_res(
            separated_list1(
                char('/'),
                many1(alt((value(true, char('#')), value(false, char('.'))))),
            ),
            |grid| {
                let nrow = grid.len();
                if nrow == 0 {
                    return Err("Empty grid");
                }
                if grid.iter().any(|row| row.len() != nrow) {
                    return Err("Non-square grid found");
                }
                let ncol = grid[0].len();

                Ok(Array2::from_shape_fn((nrow, ncol), |(i, j)| grid[i][j]))
            },
        )
        .parse_complete(input)
    }

    fn parse_transformation(input: &str) -> IResult<&str, Transformation> {
        map(
            separated_pair(Self::parse_grid, tag(" => "), Self::parse_grid),
            |(pattern, replace)| {
                let mut is_flip = true;
                let pattern = std::iter::repeat_with(|| {
                    is_flip = !is_flip;
                    is_flip
                })
                .take(8)
                .scan(pattern, |pat, is_flip| {
                    if is_flip {
                        pat.invert_axis(Axis(0));
                    } else {
                        pat.swap_axes(0, 1);
                    }
                    Some(Pattern::hash(&pat.view()))
                })
                .fold(vec![], |mut acc, pat| {
                    if !acc.contains(&pat) {
                        acc.push(pat);
                    }
                    acc
                });
                let pattern = Pattern(pattern);
                Transformation { pattern, replace }
            },
        )
        .parse_complete(input)
    }

    fn evolve(&self, n_iter: usize) -> usize {
        let mut grid = self.seed.clone();
        let mut temp;
        for _ in 0..n_iter {
            let size = grid.ncols();
            if size.is_multiple_of(2) {
                temp = Array2::from_elem((size * 3 / 2, size * 3 / 2), false);
                Zip::from(grid.exact_chunks((2, 2)))
                    .and(temp.exact_chunks_mut((3, 3)))
                    .par_for_each(|src, mut tgt| {
                        tgt.assign(&self.trans_2_to_3.transform(Pattern::hash(&src)));
                    });
            } else {
                temp = Array2::from_elem((size * 4 / 3, size * 4 / 3), false);
                Zip::from(grid.exact_chunks((3, 3)))
                    .and(temp.exact_chunks_mut((4, 4)))
                    .par_for_each(|src, mut tgt| {
                        tgt.assign(&self.trans_3_to_4.transform(Pattern::hash(&src)));
                    });
            }
            swap(&mut grid, &mut temp);
        }
        grid.into_par_iter().filter(|&&pixel| pixel).count()
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let transformations = parser::parse_input_str(
            input,
            separated_list1(line_ending, Self::parse_transformation),
        )?;
        let (trans_2_to_3, trans_3_to_4) = transformations
            .into_iter()
            .partition(|trans| trans.replace.nrows() == 3);
        let (trans_2_to_3, trans_3_to_4) = (RuleSet::new(trans_2_to_3), RuleSet::new(trans_3_to_4));
        let (_, seed) = Self::parse_grid(".#./..#/###")?;
        Ok(Self {
            trans_2_to_3,
            trans_3_to_4,
            seed,
        })
    }

    fn part1(&self) -> String {
        self.evolve(5).to_string()
    }

    fn part2(&self) -> String {
        self.evolve(18).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
