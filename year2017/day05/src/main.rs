use anyhow::Result;
use nom::{
    character::complete::{i16, line_ending},
    multi::separated_list1,
};
use util::{Solution, chore, parser};

struct Puzzle {
    jumps: Vec<i16>,
}

impl Puzzle {
    fn jump_till_end(&self, update: fn(&mut i16)) -> usize {
        let mut jumps = self.jumps.clone();
        let mut ptr = 0;
        let mut steps = 0;
        while ptr < jumps.len() {
            let val = &mut jumps[ptr];
            ptr = ptr.wrapping_add_signed(*val as isize);
            update(val);
            steps += 1;
        }
        steps
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let jumps = parser::parse_input_str(
            input,
            separated_list1(line_ending::<&str, nom::error::Error<&str>>, i16),
        )?;
        Ok(Self { jumps })
    }

    fn part1(&self) -> String {
        self.jump_till_end(|val| *val += 1).to_string()
    }

    fn part2(&self) -> String {
        self.jump_till_end(|val| match *val {
            3.. => *val -= 1,
            _ => *val += 1,
        })
        .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
